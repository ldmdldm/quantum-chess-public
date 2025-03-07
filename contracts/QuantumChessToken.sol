// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title QuantumChessToken
 * @dev ERC20 Token for the Quantum Chess ecosystem
 * @author Quantum Chess Team
 */
contract QuantumChessToken is ERC20, ERC20Burnable, Pausable, AccessControl {
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    
    // Events
    event RewardDistributed(address indexed player, uint256 amount, string reason);
    
    /**
     * @dev Constructor
     * @param initialSupply Initial token supply to mint to the deployer
     */
    constructor(uint256 initialSupply) ERC20("Quantum Chess Token", "QCT") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
        
        // Mint initial supply to the deployer
        _mint(msg.sender, initialSupply * 10 ** decimals());
    }
    
    /**
     * @dev Pause token transfers
     * @notice Only accounts with PAUSER_ROLE can call this function
     */
    function pause() public onlyRole(PAUSER_ROLE) {
        _pause();
    }
    
    /**
     * @dev Unpause token transfers
     * @notice Only accounts with PAUSER_ROLE can call this function
     */
    function unpause() public onlyRole(PAUSER_ROLE) {
        _unpause();
    }
    
    /**
     * @dev Mint new tokens
     * @param to Address to mint tokens to
     * @param amount Amount of tokens to mint
     * @notice Only accounts with MINTER_ROLE can call this function
     */
    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {
        _mint(to, amount);
    }
    
    /**
     * @dev Mint tokens as a reward for playing games
     * @param player Player address to reward
     * @param amount Amount of tokens to mint
     * @param reason Reason for the reward
     * @notice Only accounts with MINTER_ROLE can call this function
     */
    function rewardPlayer(address player, uint256 amount, string memory reason) 
        public 
        onlyRole(MINTER_ROLE) 
    {
        _mint(player, amount);
        emit RewardDistributed(player, amount, reason);
    }
    
    /**
     * @dev Batch mint tokens to multiple addresses
     * @param recipients Array of recipient addresses
     * @param amounts Array of amounts to mint to each recipient
     * @notice Only accounts with MINTER_ROLE can call this function
     */
    function batchMint(address[] memory recipients, uint256[] memory amounts) 
        public 
        onlyRole(MINTER_ROLE) 
    {
        require(recipients.length == amounts.length, "Arrays must have same length");
        
        for (uint256 i = 0; i < recipients.length; i++) {
            _mint(recipients[i], amounts[i]);
        }
    }
    
    /**
     * @dev Override _beforeTokenTransfer to check for paused state
     */
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override whenNotPaused {
        super._beforeTokenTransfer(from, to, amount);
    }
}
