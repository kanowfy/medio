import { Button, Card, Label, TextInput } from 'flowbite-react';
import { useForm } from 'react-hook-form';
import subscribeService from '../services/subscribe';

const SubscribeBox = () => {
    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm();

    const handleSubscribe = async (data, e) => {
        e.preventDefault();
        try {
            const response = await subscribeService.subscribe(data.email);
            alert("Subscribed successful");
            console.log(response);
        } catch (error) {
            console.error("Login failed:", error);
        }
    };

    return (
        <Card className="shadow-sm min-w-80">
            <form className="flex flex-col gap-4" onSubmit={handleSubmit(handleSubscribe)}>
                <div>
                    <div className="mb-2 block text-center">
                    <Label htmlFor="email" value="Subscribe to Medio newsletter" className="text-slate-700 text-lg"/>
                    </div>
                    <TextInput id="email" placeholder="johndoe123@gmail.com" required {...register("email")}/>
                </div>
                <Button type="submit">Subscribe</Button>
            </form>
        </Card>
    )
}

export default SubscribeBox;