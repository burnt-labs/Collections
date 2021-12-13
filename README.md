# Collection Metadata Standard

**Simple Summary**

An onchain standard surrounding the metadata of collections which eradicates the possibility of NFTs within the collection being replicated and other forms of market manipulation and attacks. In addition, a proposed upgrade to the Metaplex token program.

**Abstract**

The NFT marketplace is becoming increasingly vulnerable to attacks after significant growth in 2021. There are larger and larger incentives for malicious actors to create NFT replicas or claim that arbitrary NFTs belong to a given collection. The onchain collection metadata standard detailed in this document addresses the diverse risks that are surfacing as a result of rising asset values in the NFT marketplace. This standard explicitly links specific NFTs to collections and embeds key data onchain regarding the associations of NFT collections, subcollections, and NFTs. It also details an append-only array for the addition of NFTs to given collections, preventing market manipulation through the arbitrary removal of NFTs from collections.

**Motivation**

The market surrounding non-fungible tokens has recorded tremendous growth over 2021. This has brought unprecedented attention to the space, but it has also exposed various risks that exist in its current infrastructure. Vastly greater NFT values have raised the incentives for attackers to create replicas of NFT assets or create deceptive NFT collections that claim to hold assets that are not tied to the collection. The possibility to store the metadata for NFTs and NFT collections off-chain has greatly widened the scope for such attacks.

This document details an onchain metadata standard for Solana-based NFT collections which addresses the diverse risks that have emerged in the NFT marketplace. The metadata standard requires multiple onchain transactions, creating layers of security between authentic NFT assets and collections, and those created by fraudulent actors.

The high gas costs associated with the Ethereum network have increased the popularity of offchain metadata storage. The low-cost environment of the Solana chain fosters a much more favorable environment for onchain metadata storage. The risks associated with offchain metadata storage go beyond asset replication. In the case of offchain metadata storage, the value of the asset and its longevity in the marketplace depend on the server infrastructure that stores its metadata. If the server infrastructure experiences shutdowns, the authenticity, value, and ownership of the asset can be compromised.

The scope for attack has also increased due to the increasingly multichain environment and marketplace for NFT assets. While the majority of the NFT ecosystem resides on Ethereum, a growing share has been transitioning to other blockchains like Solana. As marketplaces for buying and selling these assets surface on different chains, holders will increasingly transition between chains to access these marketplaces which will change the hash of NFT collections and their respective assets. In the case that the metadata is stored off chain, verifying authenticity and provenance becomes a major challenge after such transitions.

**Specifications**

The collection metadata standard is an onchain metadata standard which stores essential information about NFTs and their associated collections on the Solana chain. This information can be used to assess the authenticity of collections and their NFTs. Decorator structs are provided to NFTs that are part of a collection. The decorator struct, collection, provides basic information about the collection to which the NFT belongs. The collection struct includes information such as the name of the collection, a description of the collection, and an array of the addresses of tokens for sub-collections that belong to the collection. Collections can belong to other collections (i.e. sub-collections). The struct also links to data which highlights all of the collections to which the NFT belongs.

Creating collections via the metadata standard requires two onchain signatures. One signature broadcasts the collection to the Solana blockchain while the other mints the NFTs or sub-collections to the collection. Further NFTs and sub-collections can be added later. The standard for minting NFTs and sub-collections is append only, meaning that NFTs and sub-collections can only be added to a collection and not removed. NFTs and subcollections are added to the members array within the NFT collections metadata.

**Case Study**

In August 2021, NFTs from the Degen Apes collection were replicated. The metadata related to the original Degen Apes was stored off chain and the replicas were almost indistinguishable from the original mints. The only factor separating the replicas from the original NFTs was 6 SOL sent to a specific address. However, the body of the transactions which received the SOL did not clearly specify where the funds were received from. The replicas managed to find themselves on secondary marketplaces, subjecting the original minters to severe potential losses. This replica could have been easily identified if the collection metadata standard noted in this document was applied.

**Considerations**

There is nothing to stop NFT collections from claiming that various NFTs belong to their collection. Users can verify that the NFT belongs to the collection by checking the collection struct of an NFTs metadata which provides information regarding all associated collections.

NFTs that wish to no longer be a part of a collection can adjust their metadata so that the reference to the collection is removed.

Collections are advised to use an append-only function for the members array which links to all NFTs and sub-collections associated with the collection. This prevents market manipulation tactics whereby NFTs are removed from collections after sales. Collections that don’t use append-only members are expected to be valued at a significant discount to those that do.

NFTs or sub-collections that are members of sub-collections are also considered to be members of the parent collection.



## Collection Metadata program

The concept of the Collection Metadata program is to provide decorator structs to a collection token. Basic info about the collection is provided with the `Collection` struct, whose account address is a PDA with a derived key of `['collection', collection_id]`.

Your wallet should be using the following information from the on-chain metadata. The [NFT Token Standard extension section](#nft-token-standard-extension) will explain how.

| Field       | Type                                     | Description                                                                                                      | 
|-------------|------------------------------------------|------------------------------------------------------------------------------------------------------------------| 
| name        | string                                   | name of the collection                                                                                           | 
| description | string                                   | short description of the collection                                                                              |
| removable   | boolean                                  | a boolean describing if elements of this collection can be removed                                               |
| expandable  | boolean                                  | a boolean describing if elements can be appended to this collection                                              |
| arrangeable | boolean                                  | a boolean describing if elements can be rearranged in this collection                                            |
| max_size    | u32                                      | an unsigned 32 bit int describing the maximum size of the collection asset list, unbounded if 0                  |
| members     | array<address>                           | an array of addresses for tokens or sub-collections belonging to this collection                                 |
| member_of   | array<[Membership Map](#membership-map)> | an array of [Membership Map](#membership-map) displaying the parent collections to which this collection belongs |


## NFT Token Standard extension

The NFT Token Standard `Metadata` struct is extended with a key `collection` which points to a map with the following fields:

| Field     | Type                                     | Description                                                                                   | Display suggestions                               | 
| --- | --- | --- | --- | 
| member_of | array<[Membership Map](#membership-map)> | array of [Membership Map](#membership-map) displaying the collections the NFT is a member of  | single NFT view, resolving links to specific NFTs |

### Membership Map

| Field     | Type   | Description                                         | Display suggestions                                                                     | 
| --- | --- |-----------------------------------------------------| --- | 
| address   | string | collection address                                  | single NFT view, links to collection and resolved collection name                       |
| signature | string | a hash of the NFT address, signed by the collection | single NFT view, resolves with collection public key and verifies validity of signature |

## Ownership

* A collection is a book-keeping record in the Metaplex ecosystem. Collections can themselves belong to collections. Collections can mint tokens and collections, both of which will automatically be signed and added to the minting collection.

### Minting new members

* Collections can mint NFTs or sub-collections, and those

* The collection's asset array is append only. Assets can be added to collections, but can not be removed.
* NFTs don't have to be minted by the collection, and can be added later.

### Adding existing NFTs

* NFTs don't have to be minted by the collection, and can be added later.
* A collection would append the NFT it is preparing to add to its `members` array.
* After the NFT address has been added to the `members` array, the NFT can call a function on the Collection to receive the verifying signature. The Collection will check for the address in the array before the signature is returned.

### Concerns / Considerations

* There is nothing stopping a collection from claiming an arbitrary NFT is a member of the collection, and showing it in its asset array. Therefore, you should always verify the signature in the asset NFTs metadata.
* Unrelated NFTs won't show the collection address and signature unless they belong to the collection.
* NFTs that choose to no longer associate with the collection may remove the `collection.member_of` array value that references the collection to be removed.
* It is highly recommended that the Collections use an append-only `members` array. This will give confidence against bait-ands-switch tactics and devaluing NFTs sold under certain conditions. It is expected that the value of a Collection that uses an append-only array will be higher than those that do not.
* Members of sub-collections are also considered members of the parent collections.

### JSON Structure example

```json
{
  "name": "Parent Collection",
  "description": "Top level example collection",
  "members": ["BxHJqGtC629c55swCqWXFGA2rRF1igbbTmh200000001"]
  
}
```

```json
{
  "name": "Middle Collection",
  "description": "Example sub-collection",
  "members": ["BxHJqGtC629c55swCqWXFGA2rRF1igbbTmh200000002"],
  "member_of": ["BxHJqGtC629c55swCqWXFGA2rRF1igbbTmh200000000"]
}
```

```json
{
  "name": "Pigs on Solana Season #1",
  "symbol": "",
  "description": "Sub-collection for Pigs on Solana. Represents the first season released.",
  "seller_fee_basis_points": 0,
  "image": "",
  "animation_url": "",
  "external_url": "https://pigsonsolana.example.com/season/1",
  "collection": {
    "member_of": [
      {
        "address": "BxHJqGtC629c55swCqWXFGA2rRF1igbbTmh200000001",
        "signature": "<signature in hex>"
      }
    ]
  }
}
```
