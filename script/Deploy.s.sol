// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Reward} from "../src/Reward.sol";
import {RewardNFT} from "../src/RewardNFT.sol";

contract DeployScript is Script {
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

    constructor() {
        setUp();
    }

    function getRewardAddress() public view returns (address) {
        return address(reward);
    }

    function getRewardNFTAddress() public view returns (address) {
        return address(rewardNFT);
    }

    function run() external {
        // Get the environment variable "PRIVATE_KEY"
        uint256 deployer = vm.envUint("PRIVATE_KEY");

        /**
         * Set intended owner as the deployer
         */
        intendedOwner = vm.addr(deployer);

        // Use that private key as the account that sends the transactions
        vm.startBroadcast(deployer);

        deploy();

        // Stop using the private key to send transactions
        vm.stopBroadcast();
    }

    function setUp() public {
        /**
         * Set intended owner as the origin
         */
        intendedOwner = msg.sender;
    }

    function deploy() public {
        /**
         * Deploy Reward with DeployScript as owner
         */
        reward = new Reward(intendedOwner);
        /**
         * Deploy RewardNFT with DeployScript as owner
         */
        rewardNFT = new RewardNFT(intendedOwner, address(reward));

        /**
         * Set RewardNFT to Reward
         */
        reward.setRewardNFT(address(rewardNFT));

        // TODO Fix permission
        // /**
        //  * Transfer ownership of Reward to intendedOwner
        //  */
        // reward.transferOwnership(intendedOwner);
        // /**
        //  * Transfer ownership of RewardNFT to intendedOwner
        //  */
        // rewardNFT.transferOwnership(intendedOwner);
    }
}
