use super::*;

pub fn render() -> Dom {
    html!("header", {
        .children([
            html!("a", {
                .class("left")
                .attr("href", "http:/hundred.su")
            }),
            html!("div", { .class("menu")
              .children(&mut GUEST_MENU_ITEMLAR.iter()
                  .map(|GuestMenuItem { caption, href}|
                      html!("a", {
                          .attr("href", href)
                          .text(caption)
                      })
                  ).collect::<Vec<_>>()
              )
            }),
            html!("div", { .class("right")
                .child( html!("a", {
                    .attr("href", "admin")
                    .class("login")
                    .text("Войти")
                }))
            }),
        ])
    })
}
