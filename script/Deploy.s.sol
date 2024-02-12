// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Reward} from "../src/Reward.sol";
import {RewardNFT} from "../src/RewardNFT.sol";

contract DeployScript is Script {
    /**
     * The deploy script is temporary owner
     */
    address tempOwner = address(this);
    /**
     * The intended owner of the deployed contracts
     */
    address public intendedOwner;
    /**
     * The Reward contract
     */
    Reward public reward;
    /**
     * The RewardNFT contract
     */
    RewardNFT public rewardNFT;

    function setUp() public {
        /**
         * Set intended owner as the sender
         */
        intendedOwner = msg.sender;
    }

    function run() public {
        /**
         * Deploy Reward with DeployScript as owner
         */
        reward = new Reward(tempOwner);
        /**
         * Deploy RewardNFT with DeployScript as owner
         */
        rewardNFT = new RewardNFT(tempOwner, address(reward));

        /**
         * Set RewardNFT to Reward
         */
        reward.setRewardNFT(address(rewardNFT));

        /**
         * Transfer ownership of Reward to intendedOwner
         */
        reward.transferOwnership(intendedOwner);
        /**
         * Transfer ownership of RewardNFT to intendedOwner
         */
        rewardNFT.transferOwnership(intendedOwner);

        /**
         * Broadcast
         */
        vm.broadcast();
    }
}