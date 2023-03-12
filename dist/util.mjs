// Turn setTimeout into a promise:
export function set_timeout(milliseconds) {
	return new Promise(res => setTimeout(res, milliseconds));
}
