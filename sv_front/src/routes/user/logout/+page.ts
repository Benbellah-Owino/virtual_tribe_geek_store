import { redirect } from "@sveltejs/kit"

export async function load(){
    document.cookie = 'auth_token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 UTC;'
	redirect(301,'/user/creator/login')
}