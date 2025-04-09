-- Your SQL goes here
CREATE TABLE installments (
  id SERIAL PRIMARY KEY,
  payment_id INTEGER NOT NULL REFERENCES payments(id) ON DELETE CASCADE,
  amount DECIMAL(10,2) NOT NULL,
  due_date DATE NOT NULL,
  paid_date DATE,
  status VARCHAR(20) NOT NULL DEFAULT 'pending',
  external_reference VARCHAR(255),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Add indexes
CREATE INDEX installments_payment_id_idx ON installments (payment_id);
CREATE INDEX installments_status_idx ON installments (status);
CREATE INDEX installments_due_date_idx ON installments (due_date);