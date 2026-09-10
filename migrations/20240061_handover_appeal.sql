-- Worker appeal/reminder: raised when a hospital hasn't approved the handover
-- within a day, nudging the hospital (and admins) to act before auto-approval.
ALTER TABLE shift_handovers
    ADD COLUMN IF NOT EXISTS appeal_raised_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS appeal_note      TEXT;
