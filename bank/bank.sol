// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Bank {
    // 记录每个地址的存款金额
    mapping(address => uint) public deposits;
    
    // 存款金额排名前 3 的用户地址
    address[3] public topUsers;
    
    // 合约管理员地址
    address public owner;

    // 构造函数，设置合约部署者为管理员
    constructor() {
        owner = msg.sender;
    }

    // 修饰符：仅限管理员调用
    modifier onlyOwner() {
        require(msg.sender == owner, "You are not the owner");
        _;
    }

    // 收款函数，允许用户通过直接转账给合约存款
    receive() external payable {
        // 更新用户的存款记录
        deposits[msg.sender] += msg.value;
        // 更新前 3 名用户记录
        _updateTopUsers(msg.sender);
    }

    // 仅管理员可以通过该方法提取资金
    function withdraw(uint amount) external onlyOwner {
        require(address(this).balance >= amount, "Insufficient contract balance");
        payable(owner).transfer(amount);
    }

    // 更新前 3 名存款最多的用户
    function _updateTopUsers(address user) internal {
        // 临时变量记录新加入用户的存款金额
        uint userDeposit = deposits[user];

        // 检查用户是否已经在前 3 名中
        for (uint i = 0; i < 3; i++) {
            if (topUsers[i] == user) {
                // 如果用户已经在前 3 名，重新排序
                _sortTopUsers();
                return;
            }
        }

        // 检查是否可以进入前 3 名
        for (uint i = 0; i < 3; i++) {
            if (deposits[topUsers[i]] < userDeposit) {
                // 将用户插入到前 3 名中，其他用户后移
                for (uint j = 2; j > i; j--) {
                    topUsers[j] = topUsers[j - 1];
                }
                topUsers[i] = user;
                break;
            }
        }
    }

    // 对前 3 名用户按存款金额进行排序
    function _sortTopUsers() internal {
        // 简单排序算法按金额从大到小排序
        for (uint i = 0; i < 2; i++) {
            for (uint j = i + 1; j < 3; j++) {
                if (deposits[topUsers[i]] < deposits[topUsers[j]]) {
                    address temp = topUsers[i];
                    topUsers[i] = topUsers[j];
                    topUsers[j] = temp;
                }
            }
        }
    }

    // 获取当前合约的余额
    function getBalance() public view returns (uint) {
        return address(this).balance;
    }
}
