// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
contract MyNFT is ERC721URIStorage {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    // Base URI for NFT metadata
    string private _baseTokenURI;

    constructor() ERC721("PingZI", "SXNFT") {}

    function _baseURI() internal view virtual override returns (string memory) {
        return _baseTokenURI;
    } 

   
    function mintNFT(string memory tokenURI) public returns (uint256) {
        uint256 tokenId = _tokenIds.current();
        _tokenIds.increment();

        _mint(msg.sender, tokenId);
        _setTokenURI(tokenId, tokenURI);

        return tokenId;
    }
 
}
 