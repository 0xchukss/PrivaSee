'use client';

import React, { useState, useCallback } from 'react';

interface ContactInputProps {
  /** Called with the validated list of wallet addresses */
  onSubmit: (contacts: string[]) => void;
  /** Whether submission is in progress */
  isSubmitting?: boolean;
  /** Max number of contacts allowed */
  maxContacts?: number;
}

/**
 * ContactInput — Multi-address input for wallet addresses.
 *
 * Users can paste or type up to 20 Solana wallet addresses.
 * Supports:
 *   - Single address entry via the input field
 *   - Bulk paste (comma or newline separated)
 *   - Individual removal
 *   - Solana address format validation (base58, 32-44 chars)
 */
export function ContactInput({
  onSubmit,
  isSubmitting = false,
  maxContacts = 20,
}: ContactInputProps) {
  const [contacts, setContacts] = useState<string[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [error, setError] = useState<string | null>(null);

  // Basic Solana address validation (base58, 32-44 characters)
  const isValidAddress = (addr: string): boolean => {
    return /^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(addr.trim());
  };

  const addContact = useCallback(
    (address: string) => {
      const trimmed = address.trim();
      if (!trimmed) return;

      if (!isValidAddress(trimmed)) {
        setError(`Invalid address: ${trimmed.slice(0, 8)}...`);
        return;
      }
      if (contacts.includes(trimmed)) {
        setError('Address already added');
        return;
      }
      if (contacts.length >= maxContacts) {
        setError(`Maximum ${maxContacts} contacts allowed`);
        return;
      }

      setContacts((prev) => [...prev, trimmed]);
      setError(null);
    },
    [contacts, maxContacts]
  );

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && inputValue.trim()) {
      e.preventDefault();
      addContact(inputValue);
      setInputValue('');
    }
  };

  const handlePaste = (e: React.ClipboardEvent<HTMLInputElement>) => {
    e.preventDefault();
    const pasted = e.clipboardData.getData('text');
    // Split by commas, newlines, or spaces
    const addresses = pasted
      .split(/[\n,\s]+/)
      .map((a) => a.trim())
      .filter(Boolean);

    let added = 0;
    for (const addr of addresses) {
      if (contacts.length + added >= maxContacts) break;
      if (isValidAddress(addr) && !contacts.includes(addr)) {
        setContacts((prev) => [...prev, addr]);
        added++;
      }
    }
    setInputValue('');
    if (added > 0) setError(null);
  };

  const removeContact = (index: number) => {
    setContacts((prev) => prev.filter((_, i) => i !== index));
    setError(null);
  };

  const handleSubmit = () => {
    if (contacts.length === 0) {
      setError('Add at least one contact');
      return;
    }
    onSubmit(contacts);
  };

  return (
    <div className="space-y-4">
      {/* Input field */}
      <div className="relative">
        <input
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
          onPaste={handlePaste}
          placeholder="Paste or type a Solana wallet address..."
          className="input-field pr-24 font-mono text-sm"
          disabled={isSubmitting}
          id="contact-input"
        />
        <button
          onClick={() => {
            addContact(inputValue);
            setInputValue('');
          }}
          disabled={!inputValue.trim() || isSubmitting}
          className="absolute right-2 top-1/2 -translate-y-1/2 px-3 py-1.5 rounded-lg text-xs font-medium bg-pv-600 text-white hover:bg-pv-500 disabled:opacity-40 disabled:cursor-not-allowed transition-all"
        >
          Add
        </button>
      </div>

      {/* Helper text */}
      <div className="flex items-center justify-between text-xs">
        <span className="text-white/30">
          Paste multiple addresses separated by commas or newlines
        </span>
        <span
          className={`font-medium ${
            contacts.length >= maxContacts ? 'text-red-400' : 'text-white/40'
          }`}
        >
          {contacts.length}/{maxContacts}
        </span>
      </div>

      {/* Error message */}
      {error && (
        <div className="flex items-center gap-2 text-red-400 text-sm">
          <svg className="w-4 h-4 flex-shrink-0" viewBox="0 0 20 20" fill="currentColor">
            <path
              fillRule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
              clipRule="evenodd"
            />
          </svg>
          {error}
        </div>
      )}

      {/* Contact list */}
      {contacts.length > 0 && (
        <div className="glass-card p-3 space-y-1.5 max-h-64 overflow-y-auto">
          {contacts.map((addr, i) => (
            <div
              key={addr}
              className="flex items-center justify-between gap-3 px-3 py-2 rounded-lg bg-surface-900/60 group hover:bg-surface-900/80 transition-colors"
            >
              <div className="flex items-center gap-2 min-w-0">
                <span className="text-xs text-white/30 w-5 text-right flex-shrink-0">
                  {i + 1}.
                </span>
                <span className="text-sm font-mono text-white/70 truncate">
                  {addr}
                </span>
              </div>
              <button
                onClick={() => removeContact(i)}
                className="flex-shrink-0 p-1 rounded-md text-white/20 hover:text-red-400 hover:bg-red-500/10 opacity-0 group-hover:opacity-100 transition-all"
                aria-label={`Remove contact ${i + 1}`}
              >
                <svg className="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
                  <path
                    fillRule="evenodd"
                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                    clipRule="evenodd"
                  />
                </svg>
              </button>
            </div>
          ))}
        </div>
      )}

      {/* Submit button */}
      <button
        onClick={handleSubmit}
        disabled={contacts.length === 0 || isSubmitting}
        className="btn-primary w-full"
        id="submit-contacts"
      >
        {isSubmitting ? (
          <>
            <svg
              className="w-4 h-4 animate-spin"
              viewBox="0 0 24 24"
              fill="none"
            >
              <circle
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                strokeWidth="3"
                strokeDasharray="31.4"
                strokeLinecap="round"
              />
            </svg>
            Encrypting & Submitting...
          </>
        ) : (
          <>
            <svg className="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
              <path
                fillRule="evenodd"
                d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z"
                clipRule="evenodd"
              />
            </svg>
            Encrypt & Register {contacts.length} Contact
            {contacts.length !== 1 ? 's' : ''}
          </>
        )}
      </button>

      {/* Privacy note */}
      <p className="text-xs text-white/25 text-center">
        Your contacts are hashed & encrypted locally before leaving your browser.
        Raw addresses are never sent to any server or blockchain.
      </p>
    </div>
  );
}
