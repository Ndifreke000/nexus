-- Optional cap on the platform fee taken per payout (kobo). NULL = no cap.
-- The fee applied on transfer is min(gross * platform_fee_percent, this cap).
ALTER TABLE platform_settings
    ADD COLUMN IF NOT EXISTS platform_fee_cap_kobo BIGINT
        CHECK (platform_fee_cap_kobo IS NULL OR platform_fee_cap_kobo >= 0);
