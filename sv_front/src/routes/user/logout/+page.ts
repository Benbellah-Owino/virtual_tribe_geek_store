export async function load(){
    document.cookie = 'auth_token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 UTC;';

    await fetch(`http://localhost:7878/logout`, {
			method: 'POST',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		})
}