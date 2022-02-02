import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Collections } from '../target/types/collections';

describe('collections', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Collections as Program<Collections>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
