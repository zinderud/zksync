var ZkSync = artifacts.require("./ZkSync.sol");
module.exports = function (deployer) {
  deployer.deploy(ZkSync);
};
