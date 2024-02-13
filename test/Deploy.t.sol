// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {DeployScript} from "../script/Deploy.s.sol";

contract DeployScriptTest is Test {
    DeployScript public script;

    function setUp() public {
        script = new DeployScript();
    }

    function testRun() public {
        script.setUp();
        script.deploy();

        assertEq(
            script.reward().owner(),
            address(this),
            "Reward contract owner is not correct"
        );
        assertEq(
            script.rewardNFT().owner(),
            address(this),
            "RewardNFT contract owner is not correct"
        );
        assertEq(
            script.reward().minter(),
            address(script.rewardNFT()),
            "Reward contract's RewardNFT is not correct"
        );
    }
}
