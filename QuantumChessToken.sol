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
    // Create a minter role for authorized minters
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    // Create a pauser role for pausing token transfers in emergency
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    // Events
    event MinterAdded(address indexed account);
    event MinterRemoved(address indexed account);
    event TokensMinted(address indexed to, uint256 amount);
    event TokensBurned(address indexed from, uint256 amount);

    /**
     * @dev Constructor for the Quantum Chess Token
     * @param initialMinter The address that gets the minter and admin roles
     */
    constructor(address initialMinter) ERC20("Quantum Chess Token", "QCT") {
        // Grant the contract deployer the default admin role (controls all roles)
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        
        // Grant the minter role to the initial minter
        _grantRole(MINTER_ROLE, initialMinter);
        
        // Grant the pauser role to the contract deployer
        _grantRole(PAUSER_ROLE, msg.sender);
        
        // Initial supply - can be 0 as tokens will be minted later
        // Alternatively, we can mint an initial supply to the deployer
        _mint(msg.sender, 1000000 * 10 ** decimals());
        
        emit MinterAdded(initialMinter);
    }

    /**
     * @dev Creates `amount` new tokens for `to`.
     * @param to The address that will receive the minted tokens
     * @param amount The amount of tokens to mint
     * 
     * Requirements:
     * - the caller must have the `MINTER_ROLE`
     */
    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {
        _mint(to, amount);
        emit TokensMinted(to, amount);
    }

    /**
     * @dev Destroys `amount` tokens from the caller.
     * @param amount The amount of tokens to burn
     * 
     * Overrides the burn function from ERC20Burnable to add event
     */
    function burn(uint256 amount) public override {
        super.burn(amount);
        emit TokensBurned(_msgSender(), amount);
    }

    /**
     * @dev Destroys `amount` tokens from `account`, deducting from the caller's allowance.
     * @param account The account whose tokens will be burnt
     * @param amount The amount of tokens to burn
     * 
     * Overrides the burnFrom function from ERC20Burnable to add event
     */
    function burnFrom(address account, uint256 amount) public override {
        super.burnFrom(account, amount);
        emit TokensBurned(account, amount);
    }

    /**
     * @dev Pauses all token transfers.
     * 
     * Requirements:
     * - the caller must have the `PAUSER_ROLE`
     */
    function pause() public onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @dev Unpauses all token transfers.
     * 
     * Requirements:
     * - the caller must have the `PAUSER_ROLE`
     */
    function unpause() public onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    /**
     * @dev Hook that is called before any transfer of tokens.
     * @param from Address sending tokens
     * @param to Address receiving tokens
     * @param amount Amount of tokens being transferred
     * 
     * Reverts if the contract is paused.
     */
    function _beforeTokenTransfer(address from, address to, uint256 amount)
        internal
        whenNotPaused
        override
    {
        super._beforeTokenTransfer(from, to, amount);
    }

    /**
     * @dev Returns the number of decimals used for token amounts
     * Standard ERC20 uses 18 decimals
     */
    function decimals() public view virtual override returns (uint8) {
        return 18;
    }

    /**
     * @dev Add a new minter. Only callable by admin.
     * @param account The address to grant the minter role
     */
    function addMinter(address account) public onlyRole(DEFAULT_ADMIN_ROLE) {
        grantRole(MINTER_ROLE, account);
        emit MinterAdded(account);
    }

    /**
     * @dev Remove a minter. Only callable by admin.
     * @param account The address to revoke the minter role from
     */
    function removeMinter(address account) public onlyRole(DEFAULT_ADMIN_ROLE) {
        revokeRole(MINTER_ROLE, account);
        emit MinterRemoved(account);
    }
}

