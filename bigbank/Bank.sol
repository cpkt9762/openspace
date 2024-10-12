// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
// 导入 IBank 接口
import "./IBank.sol";
contract Bank is IBank {
    // 存款记录
    mapping(address => uint) public deposits;

    // 管理员地址
    address public owner;

    // 构造函数，设置合约创建者为管理员
    constructor()   {
        owner = msg.sender;
    }

    // 修饰符：仅限管理员
    modifier onlyOwner() {
        require(msg.sender == owner, "You are not the owner");
        _;
    }

    // 允许接收存款的函数
    receive() external payable virtual {
        deposits[msg.sender] += msg.value;
    }

    // 提款函数，仅管理员可以调用
    function withdraw(uint amount) external override onlyOwner {
        require(address(this).balance >= amount, "Insufficient balance");
        payable(owner).transfer(amount);
    }

    // 获取合约的余额
    function getBalance() public view returns (uint) {
        return address(this).balance;
    }
}
