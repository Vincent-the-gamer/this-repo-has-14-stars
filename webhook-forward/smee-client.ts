import SmeeClient from "smee-client"
/** 
 * Get your own webhook proxy url at https://smee.io/.
 * Then create a `config.ts` besides this file, and write this:
 * ``` 
 *    export const WebhookProxyUrl: string = "your proxy url" 
 * ```
 */ 
import { WebhookProxyUrl } from "./config"

const smee = new SmeeClient({
    source: WebhookProxyUrl,
    target: 'http://localhost:8080/updateStar',
    logger: console
  })
  
const events = smee.start()

smee.onerror(() => {
    // Stop forwarding events
    events.close()
})

