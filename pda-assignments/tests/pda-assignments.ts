import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

// დაიმპორტე ყველა შენი პროგრამის ტიპი
import { ProfileSystem } from "../target/types/profile_system";
import { NoteTaking } from "../target/types/note_taking";
import { VotingSystem } from "../target/types/voting_system";

describe("PDA Assignments Testing", () => {
  // კონფიგურაცია
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // პროგრამების ინსტანციების აღება Workspace-დან
  const profileProgram = anchor.workspace.ProfileSystem as Program<ProfileSystem>;
  const noteProgram = anchor.workspace.NoteTaking as Program<NoteTaking>;
  const votingProgram = anchor.workspace.VotingSystem as Program<VotingSystem>;

  const user = provider.wallet;

  // --- 1. Profile System ტესტი ---
  describe("Profile System", () => {
    it("ქმნის პროფილს და ბლოკავს დუბლიკატს", async () => {
      const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), user.publicKey.toBuffer()],
        profileProgram.programId
      );

      await profileProgram.methods
        .initializeProfile("otari", "DevOps & Rust")
        .accounts({
          
          authority: user.publicKey,
        })
        .rpc();

      const acc = await profileProgram.account.profile.fetch(profilePda);
      expect(acc.username).to.equal("otari");

      // მცდელობა მეორედ შექმნის - უნდა ჩავარდეს
      try {
        await profileProgram.methods
          .initializeProfile("hacker", "duplicate")
          .accounts({ authority: user.publicKey })
          .rpc();
        expect.fail("უნდა დაერორებულიყო!");
      } catch (e) {
        // წარმატებაა თუ ერორი მოვიდა
      }
    });
  });

  // --- 2. Note Taking ტესტი ---
  describe("Note Taking", () => {
    it("ქმნის ნოუთს უნიკალური სათაურით", async () => {
      const title = "Homework";
      const [notePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("note"), user.publicKey.toBuffer(), Buffer.from(title)],
        noteProgram.programId
      );

      await noteProgram.methods
        .createNote(title, "Finish PDA assignments")
        .accounts({
          
          authority: user.publicKey,
        })
        .rpc();

      const acc = await noteProgram.account.note.fetch(notePda);
      expect(acc.content).to.equal("Finish PDA assignments");
    });
  });

  // --- 3. Voting System ტესტი ---
  describe("Voting System", () => {
    it("ერთი ხმა თითო პოლზე", async () => {
      const pollId = new anchor.BN(101);
      const [pollPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("poll"), pollId.toArrayLike(Buffer, "le", 8)],
        votingProgram.programId
      );

      // პოლის შექმნა
      await votingProgram.methods
        .createPoll(pollId, "Solana or ETH?")
        .accounts({
         
          authority: user.publicKey,
        })
        .rpc();

      const [votePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vote"), pollPda.toBuffer(), user.publicKey.toBuffer()],
        votingProgram.programId
      );

      // ხმის მიცემა
      await votingProgram.methods
        .castVote(1)
        .accounts({
          poll: pollPda,
        
          voter: user.publicKey,
        })
        .rpc();

      const pollAcc = await votingProgram.account.poll.fetch(pollPda);
      expect(pollAcc.totalVotes.toNumber()).to.equal(1);
    });
  });
});