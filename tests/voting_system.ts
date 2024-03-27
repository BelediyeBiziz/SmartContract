import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingSystem } from "../target/types/voting_system";
import { expect } from "chai";

const proposalUId = "c17f568d-b80d-4dce-2dcea47fae49";

describe("voting_system", () => {
  anchor.setProvider(anchor.AnchorProvider.env());


  const program = anchor.workspace.VotingSystem as Program<VotingSystem>;
  const userWallet = anchor.workspace.VotingSystem.provider.wallet;

  console.log('--->',program.programId.toBase58());
  console.info("User:", userWallet.publicKey.toBase58());

  const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode(proposalUId))],
    program.programId
  );

  const [userProposalVote] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode(proposalUId)),
      userWallet.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("Create Proposal", async () => {
    await program.methods
      .createProposal(proposalUId)
      .accounts({
        proposal: proposalPda,
        userProposalVote: userProposalVote,
        user: userWallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const proposalAccount = await program.account.proposal.fetch(proposalPda);
    expect(proposalAccount.proposalUid).to.equal(proposalUId);
    expect(proposalAccount.yesVotes.toNumber()).to.equal(0);
    expect(proposalAccount.noVotes.toNumber()).to.equal(0);
  });

  it("Vote Yes", async () => {
    const tx = await program.methods
      .voteProposal(proposalUId, { yes: {} })
      .accounts({
        proposal: proposalPda,
        userProposalVote: userProposalVote,
        user: userWallet.publicKey,
      })
      .rpc();
    console.log("Yes Tx:", tx);
    const proposalAccount = await program.account.proposal.fetch(proposalPda);
    expect(proposalAccount.yesVotes.toNumber()).to.equal(1);
    expect(proposalAccount.noVotes.toNumber()).to.equal(0);
  });

  it("Vote Yes Again", async () => {
    const tx = await program.methods
      .voteProposal(proposalUId, { yes: {} })
      .accounts({
        proposal: proposalPda,
        userProposalVote: userProposalVote,
        user: userWallet.publicKey,
      })
      .rpc();
    console.log("Yes Again Tx:", tx);
    const proposalAccount = await program.account.proposal.fetch(proposalPda);
    expect(proposalAccount.yesVotes.toNumber()).to.equal(2);
    expect(proposalAccount.noVotes.toNumber()).to.equal(0);
  });
});
