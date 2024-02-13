// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Reward} from "../src/Reward.sol";

/**
 * Attacker contract that tries to mint tokens
 */
contract Attacker {
    Reward public reward;

    constructor(Reward _reward) {
        reward = _reward;
    }

    function attack(address to, uint256 amount) public {
        reward.mint(to, amount);
    }
}

contract RewardTest is Test {
    Reward public reward;
    address public initialOwner;

    function setUp() public {
        initialOwner = address(this);
        reward = new Reward(initialOwner);
    }

    function testSetRewardNFT() public {
        /**
         * Create a new address to set as the RewardNFT
         */
        address newRewardNFT = address(0x123);

        /**
         * Set the new address as the RewardNFT
         */
        reward.setRewardNFT(newRewardNFT);

        /**
         * Get the current minter
         */
        address currentRewardNFT = reward.minter();

        /**
         * Assert that the current RewardNFT is the new RewardNFT
         */
        assertEq(
            currentRewardNFT,
            newRewardNFT,
            "setRewardNFT did not set the RewardNFT correctly"
        );
    }

    function testMint() public {
        /**
         * Get initial balance of the owner
         */
        uint256 initialBalance = reward.balanceOf(initialOwner);
        uint256 amountToMint = 1000;

        /**
         * Mint the amount to the owner
         */
        reward.mint(initialOwner, amountToMint);

        /**
         * Get final balance of the owner
         */
        uint256 finalBalance = reward.balanceOf(initialOwner);

        /**
         * Assert that the final balance is equal to the initial balance + amountToMint
         */
        assertEq(
            finalBalance,
            initialBalance + amountToMint,
            "Minting did not increase balance correctly"
        );
    }

    function testOnlyOwnerCanMint() public {
        /**
         * Create a new address that is not the owner
         */
        address notOwner = address(0x123);

        /**
         * Create a new Attacker contract that tries to mint tokens
         */
        Attacker attacker = new Attacker(reward);

        /**
         * Try to mint tokens from the notOwner address and expect it to fail
         */
        try attacker.attack(notOwner, 1000) {
            fail("Minting by not owner did not fail");
        } catch (bytes memory) {
            /**
             * The attacker contract should fail to mint.
             */
            assertTrue(true);
        }

        /**
         * Set the Attacker contract as the RewardNFT. This will ensure
         * that the reward NFT can mint tokens
         */
        reward.setRewardNFT(address(attacker));

        /**
         * The attacker should now be able to mint tokens
         */
        attacker.attack(notOwner, 1000);

        /**
         * Get final balance of the notOwner
         */
        uint256 finalBalance = reward.balanceOf(notOwner);

        /**
         * Assert that the final balance is equal to 1000
         */
        assertEq(
            finalBalance,
            1000,
            "Minting by owner did not increase balance correctly"
        );
    }
}
