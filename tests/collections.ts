import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import assert from 'assert';
import { Collections } from '../target/types/collections';

const PREFIX = "collections";

const createPda = async (seeds: Array<Uint8Array>, programId: PublicKey) => {
  return await PublicKey.findProgramAddress(
    seeds, programId);
}

describe('test collections', () => {
  const idl: anchor.Idl = JSON.parse(
    require("fs").readFileSync("./target/idl/collections.json", "utf8"));

  const utf8Encoder = new TextEncoder();

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Collections as Program<Collections>;

  // generate a new provider per test
  const creator = anchor.web3.Keypair.generate();

  // collections
  const collectionV21 = "Collection V2_1";
  const collectionV22 = "Collection V2_2";

  const collectionClient = new anchor.AccountClient(
    idl, idl.accounts[0], program.programId);

  const assetMappingClient = new anchor.AccountClient(
    idl, idl.accounts[1], program.programId
  );

  it('Created a collection', async () => {
    const getLamports = await program.provider.connection.requestAirdrop(creator.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await program.provider.connection.confirmTransaction(getLamports);

    const [collectionPDA1] = await createPda(
        [utf8Encoder.encode(PREFIX), creator.publicKey.toBytes(), utf8Encoder.encode(collectionV21)],
        program.programId);

    const [collectionPDA2] = await createPda(
        [utf8Encoder.encode(PREFIX), creator.publicKey.toBytes(), utf8Encoder.encode(collectionV22)],
        program.programId);

      // Initialize first collection
    const tx1 = await program.rpc.initializeCollection(...[collectionV21, "Collection 1"],{
      accounts: {
        creator: creator.publicKey,
        collection: collectionPDA1,
        systemProgram: SystemProgram.programId
      },
      signers: [creator]
    });
    console.log("transaction signature", tx1)

    // Init second collection
    const tx2 = await program.rpc.initializeCollection(...[collectionV22, "Collection 2"],{
      accounts: {
        creator: creator.publicKey,
        collection: collectionPDA2,
        systemProgram: SystemProgram.programId
      },
      signers: [creator]
    });
    console.log("transaction signature", tx2)


    // Pull collection account and confirm user has a collection account
    // of PDA
    const collectionAccount1 = await collectionClient.fetch(collectionPDA1);
    assert.ok(collectionAccount1.name === collectionV21, "Wrong name in collection1");

    const colAccount2 = await collectionClient.fetch(collectionPDA2);
    assert.ok(colAccount2.name === collectionV22, "Wrong name in collection2");

  });

  it('Added assets to collections', async () => {
    const [collectionPDA1] = await createPda(
        [utf8Encoder.encode(PREFIX), creator.publicKey.toBytes(), utf8Encoder.encode(collectionV21)],
        program.programId);

    const [collectionPDA2] = await createPda(
        [utf8Encoder.encode(PREFIX), creator.publicKey.toBytes(), utf8Encoder.encode(collectionV22)],
        program.programId);

    // Create a asset account
    const asset1 = anchor.web3.Keypair.generate();
    const asset2 = anchor.web3.Keypair.generate();

    // Create a pda of asset as asset_mapping account
    const [asset_mapping1] = await createPda(
      [utf8Encoder.encode(PREFIX), collectionPDA1.toBytes(), asset1.publicKey.toBytes()],
      program.programId
    );

    const [asset_mapping2] = await createPda(
      [utf8Encoder.encode(PREFIX), collectionPDA2.toBytes(), asset2.publicKey.toBytes()],
      program.programId
    );

    // Add first item into collection 1
    const tx1 = await program.rpc.addAsset("Asset1", {
      accounts: {
        assetMapping: asset_mapping1,
        collection: collectionPDA1,
        authority: creator.publicKey,
        asset: asset1.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [creator]
    });
    console.log("transaction signature", tx1)


    // Add second asset into collection 2
    const tx2 = await program.rpc.addAsset("Asset2", {
      accounts: {
        assetMapping: asset_mapping2,
        collection: collectionPDA2,
        authority: creator.publicKey,
        asset: asset2.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [creator]
    });
    console.log("transaction signature", tx2)

    const test_assets = await assetMappingClient.all();

    const asset1Account = await assetMappingClient.all([
      {
        memcmp: {
          offset: 8,
          bytes: collectionPDA1.toBase58()
        }
      },
    ]
    );

    for (let i = 0; i < test_assets.length; ++i) {
      if (test_assets[i].account.meta === 'Asset1') {
        assert.ok(test_assets[i].account.collection.toBase58() === collectionPDA1.toBase58(), "Wrong collection account on asset");
      } else {
        assert.ok(test_assets[i].account.collection.toBase58() === collectionPDA2.toBase58(), "Wrong collection account on asset");
      }
    }
    for (let i = 0; i < asset1Account.length; ++i) {
      assert.ok(asset1Account[i].account.meta === "Asset1", "Wrong collection assigned to asset");
    }
  });
});
