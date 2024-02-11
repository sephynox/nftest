// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import "../src/Reward.sol";
import "../src/RewardNFT.sol";

contract RewardNFTTest is Test {
    Reward reward;
    RewardNFT rewardNFT;
    address public initialOwner;

    function setUp() public {
        initialOwner = address(this);
        reward = new Reward(initialOwner);
        rewardNFT = new RewardNFT(initialOwner, address(reward));

        reward.setRewardNFT(address(rewardNFT));
    }

    /**
     * Mock the ERC721 receiver
     */
    function onERC721Received(address, address, uint256, bytes calldata) external pure returns(bytes4) {
        return this.onERC721Received.selector;
    }

    function testSafeMint() public {
        uint256 tokenId = 1;
        string memory uri = "https://example.com";
        uint256 rewardValue = 100;

        /**
         * Mint the NFT with the reward value
         */
        rewardNFT.safeMint(initialOwner, tokenId, uri, rewardValue);

        /**
         * Check if the NFT is minted
         */
        assertTrue(rewardNFT.checkIfTokenExist(tokenId));
        /**
         * Check if the NFT is owned by the correct address
         */
        assertEq(rewardNFT.ownerOf(tokenId), initialOwner);
        /**
         * Check if the NFT has the correct URI
         */
        assertEq(rewardNFT.tokenURI(tokenId), uri);
        /**
         * Check if the NFT has the correct reward value
         */
        assertEq(rewardNFT.getRewardValue(tokenId), rewardValue);
    }

    function testCheckIfTokenExist() public {
        uint256 tokenId = 1;

        /**
         * The NFT is not minted yet
         */
        assertFalse(rewardNFT.checkIfTokenExist(tokenId));

        /**
         * Mint the NFT
         */
        rewardNFT.safeMint(initialOwner, tokenId, "https://example.com", 100);

        /**
         * The NFT is minted
         */
        assertTrue(rewardNFT.checkIfTokenExist(tokenId));
    }

    function testBurn() public {
        /**
         * Specify the token ID
         */
        uint256 tokenId = 1;
        /**
         * Specify the reward value
         */
        uint256 rewardValue = 100;
        /**
         * Check the initial balance of the reward token
         */
        uint256 initialBalance = reward.balanceOf(initialOwner);

        /**
         * Mint the NFT with the reward value
         */
        rewardNFT.safeMint(initialOwner, tokenId, "https://example.com", rewardValue);
        /**
         * Burn the NFT
         */
        rewardNFT.burn(tokenId);

        /**
         * The NFT is burned
         */
        assertFalse(rewardNFT.checkIfTokenExist(tokenId));
        /**
         * The reward value is minted
         */
        assertEq(reward.balanceOf(initialOwner), initialBalance + rewardValue);
    }
}