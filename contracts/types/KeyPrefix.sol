// SPDX-License-Identifier: BSL-1.0
pragma solidity >=0.8.0 <0.9.0;

enum KeyPrefix {
    creator,
    operational,
    coef,
    intercept,
    commission,
    isPublic,
    owner,
    treasury,
    participationToken,
    newOwner,
    nDistributions,
    distributionInterval,
    firstDistributionDate,
    prevDistributionDate,
    index,
    isFirstDistribution,
    prepay,
    gas,
    userCurrent,
    distribution,
    setOperational,
    setRegressionCoef,
    setRegressionIntercept,
    externalTokenExists,
    poolName,
    logicVersion,
    proxyName,
    tokenSaleMarketCommission,
    tsmAddressPool,
    tsmAddressSeller,
    tsmSalePrice,
    tsmMinDivision,
    tsmApplyCommission,
    tsmLockedSale,
    baseCommissionPool,
    baseCommissionTSM,
    gasPrice,
    pmConfig,
    goalAmount,
    poolType
}