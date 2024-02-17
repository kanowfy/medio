import Layout from "./Layout";

const Create = () => {
    // grab user
    // make a form to create new
    return (
        <Layout>
            <div className="h-screen flex flex-col items-center justify-center">
                <div className="text-4xl mb-7 font-bold">Publish new article</div>
                <Card className="min-w-96">
                    <form className="flex flex-col gap-4" onSubmit={handleSubmit(handleLogin)}>
                        <div>
                            <div className="mb-2 block">
                                <Label htmlFor="username" value="Your username" />
                            </div>
                            <TextInput id="username" placeholder="johndoe123" {...register("username", {
                                required: true,
                            })} />
                            {errors.username && errors.username.type === "required" && (
                                <p className="text-red-600 text-sm">Username is required.</p>
                            )}
                        </div>
                        <div>
                            <div className="mb-2 block">
                                <Label htmlFor="password" value="Your password" />
                            </div>
                            <TextInput id="password" type="password" {...register("password", {
                                required: true,
                            })} />
                            {errors.password && errors.password.type === "required" && (
                                <p className="text-red-600 text-sm">Password is required.</p>)}
                        </div>
                        <div className="text-sm">
                            Do not have an account? <a href="/register" className="font-bold text-emerald-600">Sign up</a>
                        </div>
                        <Button type="submit">Login</Button>
                    </form>
                </Card>
            </div>
        </Layout>
    )
}

export default Create;