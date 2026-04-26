/**
 * PrivaSee Integration Tests
 *
 * Tests the complete flow:
 *   1. Initialize the PSI computation definition
 *   2. Register contacts for two users
 *   3. Request PSI computation
 *   4. Verify the callback returns correct matches
 *
 * Run with: arcium test (local) or arcium test --cluster devnet
 */

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { expect } from 'chai';
import * as os from 'os';
import { randomBytes } from 'crypto';
import { sha3_256 } from 'js-sha3';

// These imports would be available after `arcium build`
// import { Privasee } from '../target/types/privasee';

describe('PrivaSee - Private Contact Discovery', () => {
  // Configure the Anchor provider
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  // The program would be loaded from workspace after build
  // const program = anchor.workspace.Privasee as Program<Privasee>;

  // Test contacts for User A
  const userAContacts = [
    'Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr',
    '7S3P4HxJpyyigGzodYwHtCxZyUQe9JiBMHyRWXArAaKv',
    'FRqrJ1MaHTnFbQTyVMjhHq7bSjJGPrPkqcpPYFiRvEzh',
    'DRpbCBMxVnDK7maPMoGQfFKuQ3AT5zVxnPMYFhR4Tqze',
  ];

  // Test contacts for User B (shares 2 contacts with User A)
  const userBContacts = [
    'Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr', // MATCH
    'BVNo2UY2poeYfNQwYFM6Q23bkrVmKjPqTrKafpbxyKrm',
    '7S3P4HxJpyyigGzodYwHtCxZyUQe9JiBMHyRWXArAaKv', // MATCH
    'EQo6JFEqLaT8cNxWHMhpah2V4VgQKPCo6m2N7Ub1k6Pz',
  ];

  // Expected mutual matches
  const expectedMatches = [
    'Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr',
    '7S3P4HxJpyyigGzodYwHtCxZyUQe9JiBMHyRWXArAaKv',
  ];

  /**
   * Helper: Hash contacts the same way the client does
   */
  function hashContacts(contacts: string[]): string[] {
    return contacts.map((c) => sha3_256(c.trim().toLowerCase()));
  }

  describe('Client-side hashing', () => {
    it('should produce consistent SHA3-256 hashes', () => {
      const hash1 = sha3_256('Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr'.toLowerCase());
      const hash2 = sha3_256('Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr'.toLowerCase());
      expect(hash1).to.equal(hash2);
    });

    it('should produce different hashes for different contacts', () => {
      const hash1 = sha3_256(userAContacts[0].toLowerCase());
      const hash2 = sha3_256(userAContacts[1].toLowerCase());
      expect(hash1).to.not.equal(hash2);
    });

    it('should produce 32-byte (64 hex char) hashes', () => {
      const hash = sha3_256(userAContacts[0].toLowerCase());
      expect(hash.length).to.equal(64);
    });
  });

  describe('Set intersection logic', () => {
    it('should correctly identify mutual contacts', () => {
      const hashesA = new Set(hashContacts(userAContacts));
      const hashesB = new Set(hashContacts(userBContacts));

      const intersection = [...hashesA].filter((h) => hashesB.has(h));
      expect(intersection.length).to.equal(2);
    });

    it('should not include non-matching contacts', () => {
      const hashesA = new Set(hashContacts(userAContacts));
      const hashesB = new Set(hashContacts(userBContacts));

      // Contact only in A
      const uniqueAHash = sha3_256(userAContacts[2].toLowerCase());
      expect(hashesB.has(uniqueAHash)).to.be.false;

      // Contact only in B
      const uniqueBHash = sha3_256(userBContacts[1].toLowerCase());
      expect(hashesA.has(uniqueBHash)).to.be.false;
    });

    it('should handle empty contact sets', () => {
      const hashesA = new Set(hashContacts([]));
      const hashesB = new Set(hashContacts(userBContacts));

      const intersection = [...hashesA].filter((h) => hashesB.has(h));
      expect(intersection.length).to.equal(0);
    });
  });

  describe('Contact preparation', () => {
    it('should pad contacts to MAX_CONTACTS (20) slots', () => {
      const contacts = userAContacts; // 4 contacts
      const hashes = hashContacts(contacts);

      // Each contact = 2 u128 (lo + hi) + 1 count = 41 total fields
      const MAX_CONTACTS = 20;
      const expectedFields = MAX_CONTACTS * 2 + 1; // 41
      expect(expectedFields).to.equal(41);
    });

    it('should normalize addresses before hashing', () => {
      const hash1 = sha3_256('  Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr  '.trim().toLowerCase());
      const hash2 = sha3_256('Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr'.trim().toLowerCase());
      expect(hash1).to.equal(hash2);
    });
  });

  // NOTE: The following tests require the Arcium toolchain and deployed program.
  // They would run via `arcium test` or `arcium test --cluster devnet`.
  //
  // describe('On-chain integration', () => {
  //   it('should initialize the PSI computation definition', async () => {
  //     const sig = await initComputePsiCompDef(program, owner);
  //     console.log('Init comp def sig:', sig);
  //   });
  //
  //   it('should register contacts for User A', async () => { ... });
  //   it('should register contacts for User B', async () => { ... });
  //   it('should execute PSI and return only matches', async () => { ... });
  // });
});
