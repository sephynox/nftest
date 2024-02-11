// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Reward} from "../src/Reward.sol";

contract RewardScript is Script {
    Reward public reward;

    function setUp() public {}

    function run() public {
        reward = new Reward(msg.sender);
        vm.broadcast();
    }
}