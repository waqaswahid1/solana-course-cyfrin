// SPDX-License-Identifier: Apache 2
pragma solidity >=0.8.8 <0.9.0;

// https://github.com/wormhole-foundation/native-token-transfers/blob/main/evm/src/interfaces/INttToken.sol

interface INttToken {
    /// @notice Error when the caller is not the minter.
    /// @dev Selector 0x5fb5729e.
    /// @param caller The caller of the function.
    error CallerNotMinter(address caller);

    /// @notice Error when the minter is the zero address.
    /// @dev Selector 0x04a208c7.
    error InvalidMinterZeroAddress();

    /// @notice Error when insufficient balance to burn the amount.
    /// @dev Selector 0xcf479181.
    /// @param balance The balance of the account.
    /// @param amount The amount to burn.
    error InsufficientBalance(uint256 balance, uint256 amount);

    /// @notice The minter has been changed.
    /// @dev Topic0
    ///      0x0b5e7be615a67a819aff3f47c967d1535cead1b98db60fafdcbf22dcaa8fa5a9.
    /// @param newMinter The new minter.
    event NewMinter(address previousMinter, address newMinter);

    // NOTE: the `mint` method is not present in the standard ERC20 interface.
    //       If using NTT in hub-and-spoke mode, this function is required in the token contract for all spoke chains.
    //       If using NTT in burn-and-mint mode, this function is required in the token contract for all chains.
    function mint(address account, uint256 amount) external;

    // NOTE: the `setMinter` method is not present in the standard ERC20 interface.
    //       This is not a required function for integration with NTT. It is recommended to provide flexibility to change the minter in the future,
    //       or allow for setting multiple token minter addresses.
    //       There are also other legitimate ways to implement minter management, such as using role-based access control or a custom approach,
    //       as opposed to the example of using a single setter that's presented here.
    function setMinter(
        address newMinter
    ) external;

    // NOTE: NttTokens in `burn` mode require the `burn` method to be present.
    //       This method is not present in the standard ERC20 interface, but is
    //       found in the `ERC20Burnable` interface.
    function burn(
        uint256 amount
    ) external;
}

interface IERC20 {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 amount)
        external
        returns (bool);
    function allowance(address owner, address spender)
        external
        view
        returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount)
        external
        returns (bool);
}

contract NttToken is IERC20, INttToken {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(
        address indexed owner, address indexed spender, uint256 value
    );

    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    string constant public name = "CYFRIN_DEV";
    string constant public symbol = "CYFRIN_DEV";
    uint8 constant public decimals = 18;

    address public owner;
    address public minter;

    constructor() {
        owner = msg.sender;
        minter = msg.sender;
    }

    function transfer(address recipient, uint256 amount)
        external
        returns (bool)
    {
        balanceOf[msg.sender] -= amount;
        balanceOf[recipient] += amount;
        emit Transfer(msg.sender, recipient, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address sender, address recipient, uint256 amount)
        external
        returns (bool)
    {
        allowance[sender][msg.sender] -= amount;
        balanceOf[sender] -= amount;
        balanceOf[recipient] += amount;
        emit Transfer(sender, recipient, amount);
        return true;
    }

    function _mint(address to, uint256 amount) internal {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function _burn(address from, uint256 amount) internal {
        balanceOf[from] -= amount;
        totalSupply -= amount;
        emit Transfer(from, address(0), amount);
    }

    function mint(address to, uint256 amount) external {
        if (msg.sender != minter) {
            revert CallerNotMinter(msg.sender);
        }
        _mint(to, amount);
    }

    function burn(uint256 amount) external {
        uint256 bal = balanceOf[msg.sender];
        if (amount > bal) {
            revert InsufficientBalance(bal, amount);
        }
        _burn(msg.sender, amount);
    }

    function setMinter(address newMinter) external {
        require(msg.sender == owner, "not authorized");

        if (newMinter == address(0)) {
            revert InvalidMinterZeroAddress();
        }
        emit NewMinter(minter, newMinter);
        minter = newMinter;
    }
}
