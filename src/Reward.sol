// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";

/**
 * @title Reward
 * @dev Reward token which is minted by RewardNFT contract upon burn. 
 */
contract Reward is ERC20, ERC20Burnable, Ownable {
    /**
     * @dev RewardNFT contract address
     */
    address public rewardNFT;

    /**
     * @dev Constructor of Reward
     * 
     * @param initialOwner Admin of this contract
     */
    constructor(address initialOwner) 
        Ownable(initialOwner) 
        ERC20("Reward", "RWD") 
    {
        /**
         * No special logic required 
         */
    }

    /**
     * @dev Modifier to check if sender is owner or the RewardNFT
     */
    modifier onlyOwnerOrRewardNFT() {
        require(msg.sender == owner() || msg.sender == rewardNFT, "Only owner or RewardNFT can mint");
        _;
    }

    /**
     * @dev Set rewardNFT address
     * 
     * @param _rewardNFT RewardNFT contract address
     */
    function setRewardNFT(address _rewardNFT) external onlyOwner {
        rewardNFT = _rewardNFT;
    }

    /**
     * @dev Mint reward token
     */
    function mint(address to, uint256 amount) external onlyOwnerOrRewardNFT {
        _mint(to, amount);
    }
}