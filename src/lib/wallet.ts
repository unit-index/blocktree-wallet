import { invoke } from '@tauri-apps/api/core';

export async function generateMnemonic(): Promise<string> {
    return await invoke('generate_mnemonic', {});
}

export async function verifyMnemonic(phrase: string): Promise<boolean> {
    return await invoke('verify_mnemonic', { phrase });
}