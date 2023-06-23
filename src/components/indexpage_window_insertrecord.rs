use serde::Serialize;
use serde_json::json;
use yew::{prelude::*, services::ConsoleService};
use yew::services::fetch::Request;
use yew::services::fetch::{Response, FetchService, FetchTask};
use yew::format::{Json, self};

pub enum Msg {
    ToggleInsertRecord,
    ValidateInputJson(String),
    RequestCreateRecordsData,
    Ignore,
    ErrorIgnore,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowInsertRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_insert_record: bool,
    pub on_toggle_insertrecord:Callback<Msg>,

    #[prop_or_default]
    pub app_id: String,
    #[prop_or_default]
    pub card_index: String, //index_name

}

pub struct InsertRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowInsertRecordProps,
    callback_toggle_insertrecord: Callback<Msg>,
    value: String,
    json_is_valid: bool,
    fetch_task: Option<FetchTask>,

    //UNTUK MUNCULIN MODAL CONFIRMATION
    request_success: bool,
    request_fail: bool,

    //UNTUK DETECT DIA MASIH LOADING ATAU NGK DI MODAL
    loading: bool,
}

impl Component for InsertRecord {
    type Message = Msg;
    type Properties = WindowInsertRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_insertrecord: props.on_toggle_insertrecord.clone(),
            props,
            value: "".to_string(),
            json_is_valid: false,
            fetch_task: None,

            request_success: false,
            request_fail: false,

            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleInsertRecord => {
                self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord);
                true
            }

            Msg::ValidateInputJson (data) => {
                self.value = data;
                self.json_is_valid = match serde_json::from_str::<serde_json::Value>(&self.value) {
                    Ok(_) => true,
                    Err(_) => false,
                };
                // ConsoleService::info(&format!("DEBUG : value:{:?}", &self.value));
                // ConsoleService::info(&format!("DEBUG : json_is_valid:{:?}", self.json_is_valid));
                true
            }

            Msg::RequestCreateRecordsData => {
                //LOADING STATUS KE STATE
                self.loading = true;
                ConsoleService::info(&format!("DEBUG loading status : {:?}", &self.loading));

                let mut records: serde_json::Value = json!({});
                match serde_json::from_str::<Vec<serde_json::Value>>(&self.value) {
                    Ok(_) => records = serde_json::from_str::<serde_json::Value>(&self.value).unwrap(),
                    Err(error) => ConsoleService::info(&format!("Error: {}", error)),
                };

                let url = format!("https://library-api.dev-domain.site/book/{}/{}", &self.props.app_id, &self.props.card_index);
                let request = Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(Json(&records))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::Ignore
                                
                            }
                            Err(error) => {
                                Msg::ErrorIgnore
                            }
                        }
                    });
                    // self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord);
                    let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                    
                    self.fetch_task = Some(task);
                    ConsoleService::info(&format!("Getting Data.."));
                    
                true
            }

            Msg::Ignore => {
                self.loading = false;
                self.request_success = true;
                ConsoleService::info(&format!("DEBUG loading status : {:?}", &self.loading));
                // ConsoleService::info(&format!("DEBUG request success : {:?}", &self.request_success));
                true
            }

            Msg::ErrorIgnore => {
                self.loading = false;
                self.request_success = true;
                ConsoleService::info(&format!("DEBUG loading status : {:?}", &self.loading));
                // ConsoleService::info(&format!("DEBUG request fail : {:?}", &self.request_fail));
                true
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // ConsoleService::info(&format!("data MODAL card_index / Index name {:?}", self.props.card_index));
            // ConsoleService::info(&format!("data MODAL app_id {:?}", self.props.app_id));
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"INSERT NEW RECORD"}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleInsertRecord)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h6>{"Add book details with the JSON Format, containing a single object or an array of
                     objects. Each fields must be inserted, copy from the example given below: "}
                     </h6>

// FORM INPUT TEXT UNTUK EXAMPLE JSON 
                    <div>
                        <form class="record-text-input" id="example-no-submit">
                            <textarea 
                                readonly=true
                                type="text" 
                                class="insert-record" 
                                style="font-size:12px;font-weight: bold; line-height: 1.4;"
                                
                                >{"[{
    \"isbn\": \"value1\",
    \"judul\": \"value2\",
    \"penulis\": \"value3\",
    \"penerbit\": \"value4\",
    \"bahasa\": \"value5\",
    \"tanggal_terbit\": \"dd-mm-yyyy\"
}]
"}                     </textarea>
                        </form>   
                    </div> 
// FORM INPUT EXAMPLE END

                    <h6>{"Add your records here"}
                    </h6>


// FORM INPUT TEXT UNTUK EXAMPLE JSON 
                    <div class="window-submit-form">
                        <form class="record-text-input" id="submit-insertrecord">
                            <textarea 
                                type="text" 
                                class="insert-record" 
                                style="font-size:12px;font-weight: bold; line-height: 1.4;"

                                oninput = self.link.callback(|data: InputData| Msg::ValidateInputJson(data.value))
                                >
                            {""}  
                            </textarea>
                //BUTTON SUBMIT (HARUS DI FORM YANG SAMA)
                        </form>   
                    </div> 
// FORM INPUT EXAMPLE END
                {
                    if self.json_is_valid {
                        if self.loading {
                            html!{
                                <button 
                                type="submit"
                                form="submit-insertrecord"
                                class="window-confirm-button"
                                // onclick = self.link.callback(|_| Msg::RequestCreateRecordsData)
                                >
                                    <span class="loader">
                                        <span class="loader-inner">
                                        </span>
                                    </span>
                                </button>
                            }
                        } else {
                            html!{
                                <button 
                                    type="submit"
                                    form="submit-insertrecord"
                                    class="window-confirm-button"
                                    onclick = self.link.callback(|_| Msg::RequestCreateRecordsData)
    
                                    // onclick=self.link.batch_callback(|_| vec![
                                    //     Msg::ToggleInsertRecord,
                                    //     Msg::RequestCreateRecordsData,
                                    // ])
                                >
                                    { "INSERT NEW BOOK DETAILS" }
                                </button>
                            }
                        }
                        
                    } else {
                        html! {
                            <button disabled=true class="window-confirm-button">
                                {"FORM INPUT MUST BE IN JSON FORMAT!"}
                            </button> 
                        }
                    }

                }
                </div>
                
                {
                    if self.request_success {
                        html!{
                            {self.modal_success()}
                        }
                            
                    } else {
                        html!{}
                    }
                }

                {
                    if self.request_fail {
                        html!{
                            {self.modal_fail()}
                        }
                    } else {
                        html!{}
                    }
                }

            </div>
        }
    }
}
impl InsertRecord {
    fn modal_success(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"INSERT RECORD SUCCESSFUL"}</h1>
                    </div> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleInsertRecord)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }

    fn modal_fail(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <button disabled=true class="window-delete-warning-main" >
                        {"INSERT RECORD FAILED!"}
                    </button> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleInsertRecord)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }


}