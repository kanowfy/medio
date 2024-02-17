import { Button, Card, Label, TextInput } from 'flowbite-react';
import Layout from './Layout';
import userService from '../services/users';
import { useForm } from 'react-hook-form';
import { useNavigate } from 'react-router-dom';

const Register = () => {
    const navigate = useNavigate();
    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm();

    const handleRegister = async (data) => {
        try {
            const response = await userService.register(data);
            console.log(response);
            navigate("/login");
        } catch (error) {
            console.error('Register failed:', error);
        }
    };
    return (
        <Layout>
            <div className="h-screen flex flex-col items-center justify-center">
                <div className="text-4xl mb-7 font-bold">Register</div>
                <Card className="min-w-96">
                    <form className="flex flex-col gap-4" onSubmit={handleSubmit(handleRegister)}>
                        <div>
                            <div className="mb-2 block">
                                <Label htmlFor="username" value="Your username" />
                            </div>
                            <TextInput id="username" placeholder="johndoe123" {...register("username", {
                                required: true,
                                minLength: 8,
                                maxLength: 50
                            })} />
                            {errors.username && errors.username.type === "required" && (
                                <p className="text-red-600 text-sm">Username is required.</p>
                            )}
                            {errors.username && errors.username.type === "minLength" && (
                                <p className="text-red-600 text-sm">Minimum username length is 8 characters.</p>
                            )}
                            {errors.username && errors.username.type === "maxLength" && (
                                <p className="text-red-600 text-sm">Maximum username length is 50 characters.</p>
                            )}
                        </div>
                        <div>
                            <div className="mb-2 block">
                                <Label htmlFor="password" value="Your password" />
                            </div>
                            <TextInput id="password" type="password" {...register("password", {
                                required: true,
                                minLength: 8,
                                maxLength: 50
                            })} />
                            {errors.password && errors.password.type === "required" && (
                                <p className="text-red-600 text-sm">Password is required.</p>)}
                            {errors.password && errors.password.type === "minLength" && (
                                <p className="text-red-600 text-sm">Minimum password length is 8 characters.</p>
                            )}
                            {errors.password && errors.password.type === "maxLength" && (
                                <p className="text-red-600 text-sm">Maximum password length is 50 characters.</p>
                            )}
                        </div>
                        <div>
                            <div className="mb-2 block">
                                <Label htmlFor="diplayName" value="Display name" />
                            </div>
                            <TextInput id="displayName" placeholder="John Doe" {...register("displayName", {
                                required: true,
                                minLength: 8,
                                maxLength: 30
                            })} />
                            {errors.displayName && errors.displayName.type === "required" && (
                                <p className="text-red-600 text-sm">Display name is required.</p>
                            )}
                            {errors.displayName && errors.displayName.type === "minLength" && (
                                <p className="text-red-600 text-sm">Minimum display name length is 8 characters.</p>
                            )}
                            {errors.displayName && errors.displayName.type === "maxLength" && (
                                <p className="text-red-600 text-sm">Maximum display name length is 30 characters.</p>
                            )}
                        </div>
                        <div>
                            <div className="mb-2 block">
                                <Label htmlFor="email" value="Your email" />
                            </div>
                            <TextInput id="email" placeholder="johndoe123@gmail.com" {...register("email", {
                                required: true,
                                pattern: /^[^@ ]+@[^@ ]+\.[^@ .]{2,}$/
                            })} />
                            {errors.email && errors.email.type === "required" && (
                                <p className="text-red-600 text-sm">Email is required.</p>
                            )}
                            {errors.email && errors.email.type === "pattern" && (
                                <p className="text-red-600 text-sm">Invalid email.</p>
                            )}
                        </div>
                        <Button type="submit">Register</Button>
                    </form>
                </Card>
            </div>

        </Layout>
    );
}

export default Register;