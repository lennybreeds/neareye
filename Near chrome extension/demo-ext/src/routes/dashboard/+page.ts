// dashboard/+page.ts

import { browser } from '$app/environment';

export async function load({ fetch }) {
    if (browser) {
        console.log('running retrieval from storage');
        // Wrap the chrome.storage.sync.get call in a Promise
        const found_reasons = await new Promise((resolve, reject) => {
            chrome.storage.sync.get(['current_url', 'reply'], function (items) {
                console.log('Settings retrieved', items);
                if (chrome.runtime.lastError) {
                    // If there's an error, reject the promise
                    reject(chrome.runtime.lastError);
                } else {
                    // Resolve the promise with your items
                    resolve(items);
                }
            });
        });
        // Return the data in the format expected by SvelteKit load function
        return {
            props: {
                data: found_reasons,
            },
        };
    } else {
        // Handle the case when not in the browser environment (if needed)
        return {
            props: {
                data: {}, // Return empty object or sensible defaults
            },
        };
    }
}
