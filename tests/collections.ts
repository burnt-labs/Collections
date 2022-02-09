import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AccountInfo, PublicKey, SystemProgram } from '@solana/web3.js';
import assert from 'assert';
import { Collections } from '../target/types/collections';

const PREFIX = "collections";

const createPda = async (seeds: Array<Buffer>, programId: PublicKey) => {
  return await PublicKey.findProgramAddress(
    seeds, programId);
}

describe('collections', () => {
  const idl: anchor.Idl = JSON.parse(
    require("fs").readFileSync("./target/idl/collections.json", "utf8"));

  const keypair = JSON.parse(
    require("fs").readFileSync("/Users/peartes/.config/solana/id.json", "utf8"));


  const signer = anchor.web3.Keypair.fromSecretKey(Buffer.from(keypair));

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Collections as Program<Collections>;

  const creator = anchor.getProvider().wallet.publicKey;

  const collectionAccount = new anchor.AccountClient(
    idl, idl.accounts[0], program.programId);

  const assetAccount = new anchor.AccountClient(
    idl, idl.accounts[1], program.programId
  );

  it('Create Collection', async () => {
    const colName1 = "Collection V2_1";
    const colName2 = "Collection V2_2";

    const [collectionPda1] = await createPda(
      [Buffer.from(PREFIX), creator.toBuffer(), Buffer.from(colName1)], 
      program.programId);

    const [collectionPda2] = await createPda(
      [Buffer.from(PREFIX), creator.toBuffer(), Buffer.from(colName2)], 
      program.programId);

      // Initialize first collection
    const tx1 = await program.rpc.initializeCollection(...[colName1, "Collection 1"],{
      accounts: {
        creator,
        collection: collectionPda1,
        systemProgram: SystemProgram.programId
      },
      signers: [signer]
    });

    // Init second collection
    const tx2 = await program.rpc.initializeCollection(...[colName2, "Collection 2"],{
      accounts: {
        creator,
        collection: collectionPda2,
        systemProgram: SystemProgram.programId
      },
      signers: [signer]
    });

    // Pull collection account and confirm user has a collection account
    // of PDA
    const colAccount1 = await collectionAccount.fetch(collectionPda1);
    assert.ok(colAccount1.name === colName1, "Wrong name in collection1");

    const colAccount2 = await collectionAccount.fetch(collectionPda2);
    assert.ok(colAccount2.name === colName2, "Wrong name in collection2");

  });

  it('Add assets to collection', async () => {
    const colName1 = "Collection V2_1";
    const colName2 = "Collection V2_2";

    // Get a collection
    const [collectionPda1] = await createPda(
      [Buffer.from(PREFIX), creator.toBuffer(), Buffer.from(colName1)],
      program.programId);
    
    const [collectionPda2] = await createPda(
      [Buffer.from(PREFIX), creator.toBuffer(), Buffer.from(colName2)], 
      program.programId);
    
    // Create a asset account
    const asset1 = anchor.web3.Keypair.generate();
    const asset2 = anchor.web3.Keypair.generate();

    // Create a pda of asset as asset_mapping account
    const [asset_mapping1] = await createPda(
      [Buffer.from(PREFIX), collectionPda1.toBuffer(), asset1.publicKey.toBuffer()],
      program.programId
    );

    const [asset_mapping2] = await createPda(
      [Buffer.from(PREFIX), collectionPda2.toBuffer(), asset2.publicKey.toBuffer()],
      program.programId
    );

    // Add first item into collection 1
    const tx1 = await program.rpc.addAsset("Asset1", {
      accounts: {
        assetMapping: asset_mapping1,
        collection: collectionPda1,
        authority: creator,
        asset: asset1.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [signer]
    });

    // Add second asset into collection 2
    const tx2 = await program.rpc.addAsset("Asset2", {
      accounts: {
        assetMapping: asset_mapping2,
        collection: collectionPda2,
        authority: creator,
        asset: asset2.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [signer]
    });

    const test_assets = await assetAccount.all();

    const asset1Account = await assetAccount.all([
      {
        memcmp: {
          offset: 8,
          bytes: collectionPda1.toBase58()
        }
      },
    ]
    );

    for (let i = 0; i < test_assets.length; ++i) {
      if (test_assets[i].account.meta === 'Asset1') {
        assert.ok(test_assets[i].account.collection.toBase58() === collectionPda1.toBase58(), "Wrong collection account on asset");
      } else {
        assert.ok(test_assets[i].account.collection.toBase58() === collectionPda2.toBase58(), "Wrong collection account on asset");
      }
    }
    for (let i = 0; i < asset1Account.length; ++i) {
      assert.ok(asset1Account[i].account.meta === "Asset1", "Wrong collection assigned to asset");
    }
  });
});
