use serde::{Deserialize, Serialize};
use crate::{core::errors,types::{self,api, storage::enums as storage_enums}, services};

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MolliePaymentsRequest {
    // #[serde(rename = "amount[currency]")]
    amount: MollieAmountType,
    // #[serde(rename = "amount[value]")]
    // amount_value: String,
    description: String,
    #[serde(rename = "redirectUrl")]
    redirecturl: String,
    method: String
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MollieAmountType {
    currency: String,
    value: String
}

// #[derive(Default, Debug, PartialEq)]
// pub struct MolliePaymentsResponse2 {
//     resource: String,
//     id: String,
//     mode: String,
//     createdat: String,
//     amount: MollieAmountType,
//     description: String,
//     method: String,
//     metadata: String,
//     status: String,
//     iscancelable: bool,
//     expiresat: String,
//     profileid: String,
//     sequencetype: String,
//     redirecturl: String,
//     webhookurl: String,
//     settlementamount: MollieSettleMentAmount,
//     _links: MollieLinks,
// }

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize,Clone)]
pub struct MollieSettleMentAmount {
    value: String,
    currency: String
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize,Clone)]
pub struct MollieLinks {
    #[serde(rename = "self")]
    mself: MollieInternalLinks,
    checkout: Option<MollieInternalLinks>,
    dashboard: MollieInternalLinks,
    documentation: MollieInternalLinks,
    #[serde(rename = "changePaymentState")]
    changepaymentstate: Option<MollieInternalLinks>,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MollieInternalLinks {
    href: String,
    #[serde(rename = "type")]
    mtype: String,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MollieSyncDetails {
    #[serde(rename = "cardNumber")]
    cardnumber: String,
    #[serde(rename = "cardHolder")]
    cardholder: String,
    #[serde(rename = "cardAudience")]
    cardaudience: String,
    #[serde(rename = "cardLabel")]
    cardlabel: String,
    #[serde(rename = "cardCountryCode")]
    cardcountrycode: String,
    #[serde(rename = "cardSecurity")]
    cardsecurity: String,
    #[serde(rename = "failureReason")]
    failurereason: Option<String>,
    #[serde(rename = "failureMessage")]
    failuremessage: Option<String>,
    #[serde(rename = "feeRegion")]
    feeregion: Option<String>,
}

impl TryFrom<&types::PaymentsAuthorizeRouterData> for MolliePaymentsRequest  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_item: &types::PaymentsAuthorizeRouterData) -> Result<Self,Self::Error> {
        match _item.payment_method {
            storage_models::enums::PaymentMethodType::Card => get_mollie_payment_data(_item),
            _ => Err(errors::ConnectorError::NotImplemented("PaymentMethod not implemented".to_string()).into()),
        }
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct MollieAuthType {
    pub(super) api_key: String
}

impl TryFrom<&types::ConnectorAuthType> for MollieAuthType  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        todo!()
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MolliePaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<MolliePaymentStatus> for storage_enums::AttemptStatus {
    fn from(item: MolliePaymentStatus) -> Self {
        match item {
            MolliePaymentStatus::Succeeded => Self::Charged,
            MolliePaymentStatus::Failed => Self::Failure,
            MolliePaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, PartialEq,Deserialize, Serialize, Clone)]
pub struct MolliePaymentsResponse {
    resource: String,
    id: String,
    mode: String,
    #[serde(rename = "createdAt")]
    createdat: String,
    amount: MollieAmountType,
    description: String,
    method: String,
    metadata: Option<String>,
    status: MollieStatus,
    #[serde(rename = "isCancelable")]
    iscancelable: Option<bool>,
    #[serde(rename = "expiresAt")]
    expiresat: Option<String>,
    #[serde(rename = "profileId")]
    profileid: String,
    #[serde(rename = "sequenceType")]
    sequencetype: String,
    #[serde(rename = "redirectUrl")]
    redirecturl: String,
    #[serde(rename = "settlementAmount")]
    settlementamount: Option<MollieSettleMentAmount>,
    #[serde(rename = "_links")]
    links: MollieLinks,
    details: Option<MollieSyncDetails>,
    #[serde(rename = "failedAt")]
    failedat: Option<String>,
    locale: Option<String>,
    #[serde(rename = "paidAt")]
    paidat: Option<String>,
    #[serde(rename = "amountRefunded")]
    amountrefunded: Option<MollieSettleMentAmount>,
    #[serde(rename = "amountRemaining")]
    amountremaining: Option<MollieSettleMentAmount>,
    #[serde(rename = "countryCode")]
    countrycode : Option<String>,
}

impl<F,T> TryFrom<types::ResponseRouterData<F, MolliePaymentsResponse, T, types::PaymentsResponseData>> for types::RouterData<F, T, types::PaymentsResponseData> {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: types::ResponseRouterData<F, MolliePaymentsResponse, T, types::PaymentsResponseData>) -> Result<Self,Self::Error> {
        let (status, error, payment_response_data) = if let Ok((status, error, payment_response_data)) = get_mollie_payment_status(item.response.clone()) { (status, error, payment_response_data) } else { todo!() };        
        let redirection_data = services::RedirectForm {
        url: item.response.links.checkout.unwrap_or_default().href,
        method: services::Method::Post,
        form_fields: std::collections::HashMap::from([(" ".to_string()," ".to_string())]),
        //     // Self(
        //     //     .url
        //     //     .query_pairs()
        //     //     .map(|(k, v)| (k.to_string(), v.to_string())),
        //     // )
        // ),
    };
    Ok(Self {
        status: storage_enums::AttemptStatus::from(item.response.status),
        response: Ok(types::PaymentsResponseData::TransactionResponse {
            resource_id: types::ResponseId::ConnectorTransactionId(item.response.id.clone()),
            redirection_data: Some(redirection_data),
            redirect: true,
            mandate_reference: None,
            connector_metadata: None,
        }),
        ..item.data
    })
    }
}

impl From<MollieStatus> for storage_enums::AttemptStatus {
    fn from(item: MollieStatus) -> Self {
        match item {
            MollieStatus::Paid => Self::Charged,
            MollieStatus::Open => Self::Authorizing,
            MollieStatus::Failed => Self::Authorizing,
            MollieStatus::Expired => Self::AuthenticationPending,
        }
    }
}

pub fn get_mollie_payment_status(
    response: MolliePaymentsResponse,
) -> errors::CustomResult<
    (
        storage_enums::AttemptStatus,
        Option<types::ErrorResponse>,
        types::PaymentsResponseData,
    ),
    errors::ConnectorError,
> {
    let status = match response.status {
        MollieStatus::Paid => storage_enums::AttemptStatus::Charged,
        MollieStatus::Open => storage_enums::AttemptStatus::Pending,
        _ => storage_enums::AttemptStatus::Failure,
    };
    let error = None;
    let payments_response_data = types::PaymentsResponseData::TransactionResponse {
        resource_id: types::ResponseId::ConnectorTransactionId(response.id),
        redirection_data: None,
        redirect: false,
        mandate_reference: None,
        connector_metadata: None,
    };
    Ok((status,error,payments_response_data))
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum MollieStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[default]
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "expired")]
    Expired,
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct MollieRefundRequest {}

impl<F> TryFrom<&types::RefundsRouterData<F>> for MollieRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::RefundsRouterData<F>) -> Result<Self,Self::Error> {
       todo!()
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for storage_enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>> for types::RefundsRouterData<api::RSync>
{
     type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: types::RefundsResponseRouterData<api::RSync, RefundResponse>) -> Result<Self,Self::Error> {
         todo!()
     }
 }

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct MollieErrorResponse {}

fn get_mollie_payment_data(
    item: &types::PaymentsAuthorizeRouterData,
) -> Result<MolliePaymentsRequest, error_stack::Report<errors::ConnectorError>> {
    // let amount_currency = "EUR".to_string();//get_amount_data(item);
    let amount = get_amount_data(item);
    // let amount_value = "1.00".to_string();
    let description = "Its my first payment request".to_string();
    let method = "creditcard".to_string();
    let redirecturl = "https://docs.mollie.com/reference/v2/payments-api/create-payment".to_string();
    let a =MolliePaymentsRequest {
        // amount_currency,
        amount,
        description,
        redirecturl,
        method
    };

    println!("abc ----> {:?}",a);
    Ok(a)
}

fn get_amount_data(item: &types::PaymentsAuthorizeRouterData) -> MollieAmountType {
    let amount = format!("{}.00", item.request.amount);
    MollieAmountType {
        currency: item.request.currency.to_string(),
        value: amount, //item.request.amount.to_string(),
    }
}