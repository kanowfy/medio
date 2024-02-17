import { useAuth } from '../provider/AuthProvider';
import { Button, Card, Label, TextInput } from 'flowbite-react';
import Layout from './Layout';
import { useForm } from 'react-hook-form';
import userService from '../services/users';
import { useNavigate } from 'react-router-dom';

const Login = () => {
    const { login } = useAuth();
    const navigate = useNavigate();
    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm();

    const handleLogin = async (data) => {
        try {
            const response = await userService.login(data.username, data.password);
            console.log(response);
            login(response);
            navigate("/profile");
        } catch (error) {
            console.error('Login failed:', error);
        }
    };


    return (
        <Layout>
            <div className="h-screen flex flex-col items-center justify-center">
                <div className="text-4xl mb-7 font-bold">Login to Medio</div>
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
    );
};

export default Login;