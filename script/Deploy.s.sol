// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Reward} from "../src/Reward.sol";
import {RewardNFT} from "../src/RewardNFT.sol";

contract DeployScript is Script {
    Reward public reward;
    RewardNFT public rewardNFT;

    function setUp() public {}

    function run() public {
        /**
         * Deploy Reward
         */
        reward = new Reward(msg.sender);
        /**
         * Deploy RewardNFT
         */
        rewardNFT = new RewardNFT(msg.sender, address(reward));

        /**
         * Set RewardNFT to Reward
         */
        reward.setRewardNFT(address(rewardNFT));
        /**
         * Broadcast
         */
        vm.broadcast();
    }
}