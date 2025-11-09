
<main class="w-full h-full flex justify-around items-center">
	<h1 class="mb-7 text-center text-3xl font-extrabold">CREATOR REGISTER PAGE</h1>
	<center>
		<form class="form alt_bg w-8/12 rounded-lg p-3 flex_col" onsubmit={register}>
			<h3 class="float-left mb-4 text-2xl font-bold">Register</h3>
			<br />
			{#if formState.inner_state == Result.Ok && formState.target == 'form'}
				<center><p class="error main_txt text-lg font-semibold">{formState.message}</p></center>
			{:else if formState.inner_state == Result.Err && formState.target == 'form'}
				<center><p class="error text-red-400 text-lg font-semibold">{formState.message}</p></center>
			{/if}
			<div class="form_div">
				<label for="username">Username</label>
				<input type="text" name="username" id="username" bind:value={register_form.username} />
				{#if formState.inner_state == Result.Err && formState.target == 'username'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="email">Email</label>
				<input type="email" name="email" id="email" bind:value={register_form.email} />
				{#if formState.inner_state == Result.Err && formState.target == 'email'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="password">password</label>
				<input type="password" name="password" id="password" bind:value={register_form.password} />

				{#if (formState.inner_state == Result.Err && formState.target == 'password') || formState.target == 'password&confirm_password'}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>
			<div class="form_div">
				<label for="confirm_password">confirm_password</label>
				<input
					type="password"
					name="confirm_password"
					id="confirm_password"
					bind:value={register_form.confirm_password}
				/>
				{#if (formState.inner_state == Result.Err && formState.target == 'confirm_password') || formState.target == 'password&confirm_password'}
					{console.log(formState.message)}
					<p class="error text-red-500">{formState.message}</p>
				{/if}
			</div>

			<button type="submit" class="btn primary_btn w-11/12">submit</button>
			<br /><br />
			If you already have an account with us
			<u class="t text-amber-700"><a href="login">click here to login.</a></u>
		</form>
	</center>

</main>