-- File attachments for shift creation. The frontend uploads files directly to
-- Cloudinary (signed upload) and submits the resulting secure_urls here.
ALTER TABLE shifts
    ADD COLUMN IF NOT EXISTS attachment_urls JSONB NOT NULL DEFAULT '[]'::jsonb;
