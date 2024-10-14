// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface BaseERC20 {
    function transfer(address recipient, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract TokenBank {
    // ERC20 Token 合约地址
    BaseERC20 public token;

    // 每个用户的存款记录
    mapping(address => uint256) public balances;

    // 构造函数，初始化 Token 合约地址
    constructor(address _tokenAddress) {
        token = BaseERC20(_tokenAddress);
    }

    // 存款函数，用户将 Token 存入 TokenBank
    function deposit(uint256 amount) public {
        require(amount > 0, "Amount must be greater than 0");

        // 将用户的 Token 转移到合约地址
        require(token.transferFrom(msg.sender, address(this), amount), "Token transfer failed");

        // 更新用户的存款记录
        balances[msg.sender] += amount;
    }

    // 提款函数，用户可以提取他们的存款
    function withdraw(uint256 amount) public {
        require(amount > 0, "Amount must be greater than 0");
        require(balances[msg.sender] >= amount, "Insufficient balance");

        // 更新用户的存款记录
        balances[msg.sender] -= amount;

        // 将 Token 发送回用户
        require(token.transfer(msg.sender, amount), "Token transfer failed");
    }

    // 查询用户的存款余额
    function getBalance(address user) public view returns (uint256) {
        return balances[user];
    }
}