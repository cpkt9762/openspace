// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IBank.sol";

contract Admin {
    // 合约的所有者
    address public owner;

    // 构造函数，设置合约创建者为所有者
    constructor() {
        owner = msg.sender;
    }
    
    // 支持直接转账到 Admin 合约
    receive() external payable {} 

    // 修饰符：仅限所有者
    modifier onlyOwner() {
        require(msg.sender == owner, "You are not the owner");
        _;
    }

    // Admin 提款函数，调用 IBank 接口的 withdraw 方法
    function adminWithdraw(IBank bank, uint amount) public onlyOwner {
         // 检查 bank 合约的余额是否足够
        require(address(bank).balance >= amount, "Insufficient bank balance");
        
        // 执行提款操作
        bank.withdraw(amount);
    }
}
