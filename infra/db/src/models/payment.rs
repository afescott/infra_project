use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use crate::schema::{payments, installments};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = payments)]
pub struct Payment {
    pub id: i32,
    pub user_id: i32,
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub payment_method: Option<String>,
    pub description: Option<String>,
    pub external_reference: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = payments)]
pub struct NewPayment {
    pub user_id: i32,
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub payment_method: Option<String>,
    pub description: Option<String>,
    pub external_reference: Option<String>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = payments)]
pub struct UpdatePayment {
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub payment_method: Option<String>,
    pub description: Option<String>,
    pub external_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = installments)]
#[diesel(belongs_to(Payment))]
pub struct Installment {
    pub id: i32,
    pub payment_id: i32,
    pub amount: Decimal,
    pub due_date: NaiveDate,
    pub paid_date: Option<NaiveDate>,
    pub status: String,
    pub external_reference: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = installments)]
pub struct NewInstallment {
    pub payment_id: i32,
    pub amount: Decimal,
    pub due_date: NaiveDate,
    pub paid_date: Option<NaiveDate>,
    pub status: String,
    pub external_reference: Option<String>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = installments)]
pub struct UpdateInstallment {
    pub amount: Option<Decimal>,
    pub due_date: Option<NaiveDate>,
    pub paid_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub external_reference: Option<String>,
}