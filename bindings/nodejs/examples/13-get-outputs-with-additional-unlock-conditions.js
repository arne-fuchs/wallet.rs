/**
 * This example gets the outputs with additional unlock conditions */
const { OutputsToCollect } = require('../out/types');
const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        const manager = await getUnlockedManager();
        const account = await manager.getAccount('Bob');
        
        await account.sync();
        const outputs = await account.getOutputsWithAdditionalUnlockConditions(OutputsToCollect.All)
        console.log('Outputs:', outputs);
    } catch (error) {
        console.log('Error: ' + error);
    }
    process.exit(0);
}

run();