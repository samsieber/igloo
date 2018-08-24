# TODO

 * Add builder impl for the request object
     * Make it chainable for easy user use
     * Make it predictable for easy machine use
	* Builder (required attrs) -> optional values setter ( -> required child setter) -> Request Node
 * Clean up jade macro
    * Merge first child and next children paths
    * Get rid of trailing comma requirements
    * Move it somewhere better than "exp"
 * Add proper vision map / readme:
    * Inspired by didact / react
    * Used for building a view layer separate from the renderer
    * Emits diffs when updating (to be consumed by user)
    * Wants to have way to set object keys (implictly uses node index / type right now
    * Wants to have way to refresh state at an arbitrary point
    * Wants to have way to pass functions around easily (belonging to impl)
