import { Button, Card, Label, TextInput, Textarea } from 'flowbite-react';
import { useForm } from 'react-hook-form';
import articleService from '../services/articles';

const Publish = (props) => {
    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm();

    const handlePublish = async (data, e) => {
        e.preventDefault();
        try {
            const response = await articleService.create(props.token, {
                authorId: Number(props.id),
                title: data.title,
                content: data.content
            });
            console.log(response);
            alert("Article published successful");
        } catch (e) {
            console.error("Publish failed: " + e);
        }
    }

    return (
        <>
                <Card className="w-full shadow-none">
                    <div className="text-3xl mt-10 mb-7 mx-10 font-bold">Publish new article</div>
                    <form className="flex flex-col gap-4 mx-10" onSubmit={handleSubmit(handlePublish)}>
                        <div>
                            <div className="mb-2 block">
                                <Label htmlFor="title" value="Title" />
                            </div>
                            <TextInput id="title" {...register("title", {
                                required: true,
                            })} />
                            {errors.title && errors.title.type === "required" && (
                                <p className="text-red-600 text-sm">Title is required.</p>
                            )}
                        </div>
                        <div>
                            <div className="mb-2 block">
                                <Label htmlFor="content" value="Content" />
                            </div>
                            <Textarea id="content" required rows={15} {...register("content", {
                                required: true,
                            })} />
                            {errors.content && errors.content.type === "required" && (
                                <p className="text-red-600 text-sm">Content is required.</p>)}
                        </div>
                        <div className="flex flex-col items-center">
                        <Button type="submit" className="w-32">Publish</Button>
                        </div>
                    </form>
                </Card>

        </>
    )
}

export default Publish;