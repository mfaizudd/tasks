import Head from "next/head";

interface LayoutProps {
    title?: string;
    children?: React.ReactNode;
}

export default function Layout(props: LayoutProps) {
    let title = "Tasks app";
    if (props.title) {
        title = props.title + " | " + title;
    }
    return (
        <>
            <Head>
                <title>{title}</title>
            </Head>
            <div>{props.children}</div>
        </>
    );
}
