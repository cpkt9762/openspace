// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./Bank.sol";

contract BigBank is Bank {
    // 修饰符：存款金额必须大于 0.001 ETH
    modifier minimumDeposit() {
        require(msg.value > 0.001 ether, "Deposit must be greater than 0.001 ether");
        _;
    }

    // 管理员转移功能
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "New owner is the zero address");
        owner = newOwner;
    }

    // 重写存款功能，增加存款限制
    receive() external payable override  minimumDeposit {
        deposits[msg.sender] += msg.value;
    }
    
}
