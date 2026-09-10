-- Image attachments for handover notes. The frontend uploads directly to
-- Cloudinary (signed upload) and submits the resulting secure_urls here.
ALTER TABLE shift_handovers
    ADD COLUMN IF NOT EXISTS image_urls JSONB NOT NULL DEFAULT '[]'::jsonb;
