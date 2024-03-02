export const prerender = false;

import type { APIRoute } from "astro";

type data = {
  id_todo?: number;
  title: string;
  status?: "Open" | "Close";
}

type req = {
  method: "GET" | "POST" | "PATCH" | "DELETE";
  data?: data;
}

export const POST: APIRoute = async ({ request }) => {
  const req = await request.json() as req;

  const todos = await fetch("http://localhost:3000/api/todos", {
  	method: req.method,
  	cache: 'no-cache',
  	headers: {
  		'Content-Type': 'application/json',
  		'X-Auth-Token': '123',
  	},
  	body: JSON.stringify(req.data),
  });
  const data = await todos.json();

  console.log(data);
  /*try {
    const res = await fetch("http://localhost:3000/todos", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    if (!res.ok) {
      throw new Error("yikes");
    }
    const resp = await res.json();
    if (resp) {
      return new Response(
        JSON.stringify({
          message: "success",
        }),
        {
          status: 200,
        }
      );
    } else {
      throw new Error("yikes");
    }
  } catch (e) {
    if (e instanceof Error) {
      console.error(e.message);
    } else {
      console.error(e);
    }
    return new Response(
      JSON.stringify({
        message: "There was an error",
      }),
      {
        status: 400,
      }
    );
  }*/

  return new Response(
    JSON.stringify({
      message: "This was a POST!",
    }),
    {status: 200,}
  );
};
