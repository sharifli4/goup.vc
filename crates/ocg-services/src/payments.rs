//! Payments service management and provider integrations.
//!
//! OCG uses a provider-agnostic payments domain model while running with a
//! single configured payments provider at a time. Provider-specific behavior
//! should stay isolated inside provider modules, while shared checkout,
//! refund, and webhook flows stay generic.

mod manager;
mod notification_composer;
mod provider;
mod webhook_reconciler;

pub use manager::{
    ApproveRefundRequestInput, DynPaymentsManager, HandleWebhookError, PgPaymentsManager,
    RejectRefundRequestInput, RequestRefundInput,
};
pub use provider::{
    CheckoutSession, CreateCheckoutSessionInput, DynPaymentsProvider, PaymentsWebhookEvent,
    RefundPaymentInput, build_payments_provider,
};

#[cfg(any(test, feature = "test-utils"))]
pub use manager::MockPaymentsManager;
#[cfg(any(test, feature = "test-utils"))]
pub use provider::{MockPaymentsProvider, PaymentsProvider, RefundPaymentResult};
