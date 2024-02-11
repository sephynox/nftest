// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";
import "./Reward.sol";

/**
 * @title RewardNFT
 * @dev A reward NFT that can be minted and burned. Burning the NFT will mint 
 * the reward ERC20 tokens to the owner of the NFT. An ERC721 token was chosen
 * for simplicity, however, this could also be an ERC1155 token.
 */
contract RewardNFT is ERC721, ERC721URIStorage, ERC721Burnable, Ownable {
    /**
     * @dev The reward token that will be minted when the NFT is burned.
     */
    Reward private rewardToken;
    /**
     * @dev The reward value of the NFT specifies how many ERC20 tokens will be 
     * minted when the NFT is burned.
     */
    mapping (uint256 => uint256) private tokenRewardValues;

    /**
     * @dev constructor
     * 
     * @param initialOwner address of the owner
     */
    constructor(
        address initialOwner,
        address _rewardToken
    ) 
        ERC721("RewardNFT", "RWNFT") 
        Ownable(initialOwner) 
    {
        rewardToken = Reward(_rewardToken);
    }

    /**
     * @dev Modifier to check if the token exists
     */
    modifier tokenExists(uint _tokenId) {
        require(checkIfTokenExist(_tokenId), "Error: Token does not exist!");
        _;
    }

    /**
     * @dev Modifier to check if the caller is the owner or the token owner
     */
    modifier onlyOwnerOrTokenOwner(uint256 tokenId) {
        address tokenOwner = ownerOf(tokenId);
        require(tokenOwner == _msgSender() || _msgSender() == owner(), "Caller is not owner nor the contract owner");
        _;
    }

    /**
     * @dev Check if the token exists
     * 
     * @param _tokenId id of the NFT to be checked
     */
    function checkIfTokenExist(uint _tokenId) public view returns(bool) {
        return (_ownerOf(_tokenId) != address(0));
    }
    
    /**
     * @dev Get the reward value of the NFT
     * 
     * @param tokenId id of the NFT
     */
    function getRewardValue(uint256 tokenId) public view tokenExists(tokenId) returns (uint256) {
        return tokenRewardValues[tokenId];
    }

    /**
     * @dev Mint the NFT
     * 
     * @param to address of the owner of the NFT
     * @param tokenId id of the NFT to be minted
     * @param uri URI of the NFT to be minted
     * @param value reward value of the NFT
     */
    function safeMint(
        address to, 
        uint256 tokenId, 
        string memory uri,
        uint256 value
    )
        public
        onlyOwner
    {
        /**
         * Mint the NFT
         */
        _safeMint(to, tokenId);
        /**
         * Set the token URI
         */
        _setTokenURI(tokenId, uri);
        /**
         * Set the reward value
         */
        _setRewardValue(tokenId, value);
    }

    /**
     * @dev Set the reward value of the NFT
     * 
     * @param tokenId id of the NFT
     * @param rewardValue reward value of the NFT
     */
    function _setRewardValue(uint256 tokenId, uint256 rewardValue) internal {
        tokenRewardValues[tokenId] = rewardValue;
    }

    // Override for ERC721URIStorage
    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }

    // Override for ERC721URIStorage
    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }

    // Override for ERC721Burnable
    function burn(uint256 tokenId) 
        public 
        virtual 
        override(ERC721Burnable)
        onlyOwnerOrTokenOwner(tokenId)
    {
        /**
         * When burning the NFT, we will want to mint the reward
         * ERC20 tokens to the owner of the NFT
         */
        address owner = ownerOf(tokenId);

        /**
         * Burn the NFT
         */
        _burn(tokenId);

        /**
         * Mint reward tokens to the owner of the NFT
         */
        rewardToken.mint(owner, 100);
    }
}