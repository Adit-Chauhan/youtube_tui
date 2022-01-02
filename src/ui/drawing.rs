use crate::util_macro::*;

use std::panic;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color::*, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::utils::video_desc;

macro_rules! space_splitter {
    (HP $space:expr, $x:expr) => {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints($x.as_ref())
            .split($space)
    };
    (VP $space:expr, $x:expr) => {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints($x.as_ref())
            .split($space)
    };
}

use super::{App, Contents};
impl App {
    pub(super) fn draw_vids<B: Backend>(&mut self, ff: Rect, f: &mut Frame<B>, heading: &str) {
        let current_title = self.titles.state.selected();
        let videos = match &self.content {
            Contents::Vid(x) => x,
            Contents::Chan(_) => panic!("This should not have been called"),
        };

        // Splitting The Screen
        let chunks =
            space_splitter!(HP ff, [Constraint::Percentage(60), Constraint::Percentage(40)]);
        let thumb_desc =
            space_splitter!(VP chunks[1],[Constraint::Percentage(50), Constraint::Percentage(50)]);

        // Creating The block contents
        let vids: Vec<ListItem> = iter_collect!(videos, |v| -> ListItem {
            ListItem::new(v.title.clone()).style(Style::default().fg(White))
        });
        let vids = List::new(vids)
            .block(Block::default().borders(Borders::ALL).title(heading))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        let vid_desc = match current_title {
            Some(x) => video_desc(&videos[x]),
            None => "".to_string(),
        };

        let vid_d = Paragraph::new(vid_desc)
            .style(Style::default().fg(White))
            .alignment(tui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(White))
                    .title("Video Description"),
            )
            .wrap(Wrap { trim: false });

        let thumb = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(White));

        self.disp_video(current_title.unwrap_or(0), &thumb_desc[0]);
        f.render_widget(vid_d, thumb_desc[1]);
        f.render_widget(thumb, thumb_desc[0]);
        f.render_stateful_widget(vids, chunks[0], &mut self.titles.state);
    }

    pub(super) fn draw_channel<B: Backend>(&mut self, ff: Rect, f: &mut Frame<B>) {
        let current_title = self.titles.state.selected();
        // Para Graph lambda
        let make_para = |cont: String| {
            Paragraph::new(cont)
                .alignment(tui::layout::Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(White)),
                )
                .wrap(Wrap { trim: true })
        };
        // Block Lambda
        let block_maker = |ti: &str| {
            Block::default()
                .style(Style::default().fg(White))
                .borders(Borders::ALL)
                .title(ti.to_string())
        };
        // empty box
        let dumb = || block_maker("");
        // Splitting Terminal
        let list_channel =
            space_splitter!(HP ff,[Constraint::Percentage(30), Constraint::Percentage(70)]);
        let desc_vids = space_splitter!(VP list_channel[1],[Constraint::Percentage(20), Constraint::Percentage(80)]);
        let thumb_desc = space_splitter!(HP desc_vids[0],[Constraint::Percentage(50), Constraint::Percentage(50)]);
        let vids_split = space_splitter!(VP desc_vids[1],[
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]);
        let indiv_vid_splits: Vec<Vec<Rect>> = iter_collect!(into vids_split,|vs| -> Vec<Rect> {
            space_splitter!(HP vs,[Constraint::Percentage(80), Constraint::Percentage(20)])
        });

        let channels = match &self.content {
            Contents::Chan(x) => x,
            Contents::Vid(_) => panic!("This should not have been called"),
        };
        let chans: Vec<ListItem> = iter_collect!(channels, |v| -> ListItem {
            ListItem::new(v.name.clone()).style(Style::default().fg(White))
        });
        let chans = List::new(chans)
            .block(block_maker("Channels"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        let chan_desc = match current_title {
            Some(x) => &channels[x].description,
            None => "",
        };
        let chan_desc = Paragraph::new(chan_desc)
            .alignment(tui::layout::Alignment::Left)
            .block(block_maker("Channel Description"))
            .wrap(Wrap { trim: true });

        let mut paras: Vec<Paragraph> = vec![Paragraph::new(""); 5];
        if let Some(x) = current_title {
            let chan_vids = &channels[x].load_videos();
            for i in 0..5 {
                if let Some(x) = chan_vids.get(i) {
                    paras[i] = make_para(x.title.clone());
                    self.draw_video_thumb(x, &indiv_vid_splits[i][1], &format!("{}", i));
                }
            }
        }

        f.render_stateful_widget(chans, list_channel[0], &mut self.titles.state);
        f.render_widget(chan_desc, thumb_desc[1]);
        f.render_widget(dumb(), thumb_desc[0]);
        for i in 0..5 {
            f.render_widget(paras[i].to_owned(), indiv_vid_splits[i][0]);
            f.render_widget(dumb(), indiv_vid_splits[i][1]);
        }
    }

    pub(super) fn draw_history<B: Backend>(&mut self, ff: Rect, f: &mut Frame<B>) {
        let videos = match &self.content {
            Contents::Vid(v) => v,
            Contents::Chan(_) => panic!("unreachable Code"),
        };

        let vids: Vec<ListItem> = iter_collect!(videos, |v| -> ListItem {
            ListItem::new(format!("{}      Channel: {} ", v.title, v.channel))
                .style(Style::default().fg(White))
        });
        let vids = List::new(vids)
            .block(Block::default().borders(Borders::ALL).title("Today"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(vids, ff, &mut self.titles.state);
    }

    pub(super) fn draw_video<B: Backend>(&mut self, ff: Rect, f: &mut Frame<B>) {
        let block_maker = |ti: &str| {
            Block::default()
                .style(Style::default().fg(White))
                .borders(Borders::ALL)
                .title(ti.to_string())
        };
        // empty box
        let dumb = || block_maker("");

        let blob_rec =
            space_splitter!(HP ff,[Constraint::Percentage(75),Constraint::Percentage(35)]);
        let vid_comment =
            space_splitter!(VP blob_rec[0],[Constraint::Percentage(30),Constraint::Percentage(70)]);
    }
}
